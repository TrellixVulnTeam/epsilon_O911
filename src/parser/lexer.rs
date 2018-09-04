use crate::interner::{Context as Intern, InternedString};
use unicode_xid::UnicodeXID;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'a> {
  Eof,
  KeywordFunc,
  Identifier(InternedString<'a>),
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
      Some((idx, ch)) => panic!(
        "Unrecognized character {} ({:x}) at index {}",
        ch, ch as u32, idx
      ),
    }
  }
}
