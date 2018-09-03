use crate::interner::{self, InternedString};
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

pub struct Lexer<'i, 's> {
  intern: &'i interner::Context,
  buffer: &'s str,
  iter: std::iter::Peekable<std::str::CharIndices<'s>>,
}

fn is_xid_start(ch: char) -> bool {
  UnicodeXID::is_xid_start(ch)
}
fn is_xid_continue(ch: char) -> bool {
  UnicodeXID::is_xid_continue(ch)
}

impl<'i, 's> Lexer<'i, 's> {
  pub fn new(program: &'s str, intern: &'i interner::Context) -> Self {
    Lexer {
      intern,
      buffer: program,
      iter: program.char_indices().peekable(),
    }
  }

  fn match_keyword(&self, first: usize, last: usize) -> Token<'i> {
    let id = self.intern.add_string(&self.buffer[first..last]);
    match id.as_str() {
      "func" => Token::KeywordFunc,
      _ => Token::Identifier(id),
    }
  }

  pub fn next_token(&mut self) -> Token<'i> {
    match self.iter.next() {
      Some((start, ch)) if is_xid_start(ch) => loop {
        match self.iter.peek() {
          Some(&(idx, ch)) if is_xid_continue(ch) => self.iter.next(),
          Some(&(idx, _)) => return self.match_keyword(start, idx),
          None => return self.match_keyword(start, self.buffer.len()),
        };
      },
      Some((start, ch)) if ch.is_whitespace() => {
        loop {
          match self.iter.peek() {
            Some(&(_, ch)) if ch.is_whitespace() => self.iter.next(),
            _ => return self.next_token(),
          };
        }
      }
      None => Token::Eof,
      _ => panic!(),
    }
  }
}
