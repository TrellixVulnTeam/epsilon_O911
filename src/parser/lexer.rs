use crate::interner::{Context as Intern, InternedString};
use unicode_xid::UnicodeXID;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
  Eof,
  KeywordFunc,
  Identifier(InternedString<'a>),
  IntegerLiteral(u64),
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
}

pub struct Lexer<'s> {
  buffer: &'s str,
  iter: std::iter::Peekable<std::str::CharIndices<'s>>,
}

fn is_xid_start(ch: char) -> bool {
  UnicodeXID::is_xid_start(ch)
}
fn is_xid_continue(ch: char) -> bool {
  UnicodeXID::is_xid_continue(ch)
}
fn base_of_letter(ch: char) -> Option<u32> {
  match ch {
    'b' | 'B' => Some(2),
    'o' | 'O' => Some(8),
    'd' | 'D' => Some(10),
    'x' | 'X' => Some(16),
    _ => None,
  }
}

impl<'s> Lexer<'s> {
  pub fn new(program: &'s str) -> Self {
    Lexer {
      buffer: program,
      iter: program.char_indices().peekable(),
    }
  }

  fn match_keyword<'i>(
    &self,
    intern: &'i Intern,
    first: usize,
    last: usize,
  ) -> Token<'i> {
    let id = intern.add_string(&self.buffer[first..last]);
    match id.as_str() {
      "func" => Token::KeywordFunc,
      _ => Token::Identifier(id),
    }
  }

  fn lex_number<'i>(
    &mut self,
    first: usize,
    base: u32,
  ) -> Token<'i> {
    let buffer = &self.buffer;
    let len = buffer.len();
    let helper = move |last| {
      Token::IntegerLiteral(
        u64::from_str_radix(&buffer[first..last], base).unwrap(),
      )
    };
    loop {
      match self.iter.peek() {
        Some(&(idx, ch)) if ch.is_digit(base) => { self.iter.next(); },
        Some(&(idx, ch)) if is_xid_continue(ch) => {
          panic!("add whitespace after numbers, before identifier characters")
        }
        Some(&(idx, _)) => return helper(idx),
        None => return helper(self.buffer.len()),
      }
    }
  }

  pub fn next_token<'i>(&mut self, intern: &'i Intern) -> Token<'i> {
    match self.iter.next() {
      None => Token::Eof,
      Some((_, '(')) => Token::OpenParen,
      Some((_, ')')) => Token::CloseParen,
      Some((_, '{')) => Token::OpenBrace,
      Some((_, '}')) => Token::CloseBrace,
      Some((start, ch)) if ch.is_whitespace() => loop {
        match self.iter.peek() {
          Some(&(_, ch)) if ch.is_whitespace() => self.iter.next(),
          _ => return self.next_token(intern),
        };
      },
      Some((start, ch)) if is_xid_start(ch) => loop {
        match self.iter.peek() {
          Some(&(idx, ch)) if is_xid_continue(ch) => self.iter.next(),
          Some(&(idx, _)) => return self.match_keyword(intern, start, idx),
          None => return self.match_keyword(intern, start, self.buffer.len()),
        };
      },
      Some((_, '0')) => match self.iter.peek() {
        Some(&(start, ch)) => {
          self.iter.next();

          if let Some(base) = base_of_letter(ch) {
            match self.iter.next() {
              Some((start, ch)) if ch.is_digit(base) => {
                self.lex_number(start, base)
              }
              Some((_, ch)) => {
                panic!(
                  "Invalid integral literal with base {}; found {}",
                  base,
                  ch,
                )
              }
              None => panic!("Unexpected EOF"),
            }
          } else if ch.is_digit(10) {
            self.lex_number(start, 10)
          } else if is_xid_continue(ch) {
            panic!("add whitespace after numbers, before identifier characters")
          } else {
            Token::IntegerLiteral(0)
          }
        }
        _ => Token::IntegerLiteral(0),
      },
      Some((idx, ch)) => panic!(
        "Unrecognized character {} ({:x}) at index {}",
        ch, ch as u32, idx
      ),
    }
  }
}
