use super::Context;

use crate::interner::Interned;
use crate::string::NfcString;
use unicode_xid::UnicodeXID;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Token<'cx> {
  Eof,

  IntegerLiteral(u64),
  StringLiteral(StringKind, &'cx str),

  Operator(Interned<'cx, NfcString>),
  Identifier(Interned<'cx, NfcString>),

  Arrow,
  KeywordFunc,
  KeywordExtern,
  KeywordUnderscore,

  Colon,
  Semicolon,
  OpenParen,
  CloseParen,
  OpenBrace,
  CloseBrace,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum StringKind {
  Normal,
  CString,
}

pub struct Lexer<'s> {
  buffer: &'s str,
  iter: std::iter::Peekable<std::str::CharIndices<'s>>,
}

fn is_operator_start(ch: char) -> bool {
  const OPERATOR_START: &[char] = &['-'];

  OPERATOR_START.contains(&ch)
}
fn is_operator_continue(ch: char) -> bool {
  const OPERATOR_CONTINUE: &[char] = &['>'];

  is_operator_start(ch) || OPERATOR_CONTINUE.contains(&ch)
}

fn is_ident_start(ch: char) -> bool {
  ch == '_' || UnicodeXID::is_xid_start(ch)
}
fn is_ident_continue(ch: char) -> bool {
  ch == '-' || ch == '\'' || UnicodeXID::is_xid_continue(ch)
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

  fn match_identifier<'cx>(
    &self,
    ctxt: &'cx Context,
    first: usize,
    last: usize,
  ) -> Token<'cx> {
    let ident = ctxt.get_ident(&self.buffer[first..last]);
    match ident.as_str() {
      "_" => Token::KeywordUnderscore,
      "func" => Token::KeywordFunc,
      "extern" => Token::KeywordExtern,
      _ => Token::Identifier(ident),
    }
  }

  fn match_operator<'cx>(
    &self,
    ctxt: &'cx Context,
    first: usize,
    last: usize,
  ) -> Token<'cx> {
    let ident = &self.buffer[first..last];
    match ident {
      "->" => Token::Arrow,
      _ => {
        let id = ctxt.get_ident(ident);
        Token::Operator(id)
      }
    }
  }

  fn lex_number<'cx>(&mut self, first: usize, base: u32) -> Token<'cx> {
    let buffer = &self.buffer;
    let helper = move |last| {
      let buff = &buffer[first..last];
      let int = match u64::from_str_radix(buff, base) {
        Ok(i) => i,
        Err(e) => panic!("ICE: invalid integer literal: {} ({})", buff, e),
      };
      Token::IntegerLiteral(int)
    };
    loop {
      match self.iter.peek() {
        Some(&(_, ch)) if ch.is_digit(base) => {
          self.iter.next();
        }
        Some(&(_, ch)) if is_ident_continue(ch) => {
          panic!("add whitespace after numbers, before identifier characters")
        }
        Some(&(idx, _)) => return helper(idx - 1),
        None => return helper(self.buffer.len()),
      }
    }
  }

  /*
    note: the iterator should be pointed at the character after the leading `"`
    i.e., for "Hello", the iterator should be at
               ^
  */
  fn lex_string<'cx>(
    &mut self,
    kind: StringKind,
    ctxt: &'cx Context,
  ) -> Token<'cx> {
    let start = match self.iter.peek() {
      Some(&(idx, _)) => idx,
      None => panic!("Unexpected EOF"),
    };

    loop {
      match self.iter.next() {
        Some((last, '"')) => {
          let s = &self.buffer[start..last];
          return Token::StringLiteral(kind, ctxt.get_string_literal(s));
        }
        Some((_, '\\')) => panic!("escapes not yet supported"),
        Some(_) => (),
        None => panic!("Unexpected EOF"),
      }
    }
  }

  /*
    note: put the iterator after the % of the opening
    [% comment comment comment %]
      ^ here
  */
  fn block_comment(&mut self) {
    loop {
      match self.iter.next() {
        Some((_, '%')) => match self.iter.peek() {
          Some((_, ']')) => {
            self.iter.next();
            break
          },
          _ => (),
        },
        Some((_, '[')) => match self.iter.peek() {
          Some((_, '%')) => {
            self.iter.next();
            self.block_comment()
          }
          _ => (),
        },
        _ => (),
      }
    }
  }

  pub fn next_token<'cx>(&mut self, ctxt: &'cx Context) -> Token<'cx> {
    match self.iter.next() {
      None => Token::Eof,
      Some((_, ':')) => Token::Colon,
      Some((_, ';')) => Token::Semicolon,
      Some((_, '(')) => Token::OpenParen,
      Some((_, ')')) => Token::CloseParen,
      Some((_, '{')) => Token::OpenBrace,
      Some((_, '}')) => Token::CloseBrace,
      Some((_, '[')) => {
        match self.iter.peek() {
          Some((_, '%')) => {
            self.iter.next();
            self.block_comment();
            self.next_token(ctxt)
          }
          _ => panic!("`[` and `]` are not yet supported")
        }
      }
      Some((_, '"')) => self.lex_string(StringKind::Normal, ctxt),
      Some((_, ch)) if ch.is_whitespace() => loop {
        match self.iter.peek() {
          Some(&(_, ch)) if ch.is_whitespace() => self.iter.next(),
          _ => return self.next_token(ctxt),
        };
      },
      Some((start, ch)) if is_ident_start(ch) => loop {
        match self.iter.peek() {
          Some(&(_, ch)) if is_ident_continue(ch) => self.iter.next(),
          Some(&(idx, '"')) => {
            self.iter.next();
            let kind = match &self.buffer[start..idx] {
              "c" | "C" => StringKind::CString,
              other => panic!("Unsupported string literal prefix: `{}`", other),
            };
            return self.lex_string(kind, ctxt);
          }
          Some(&(idx, _)) => return self.match_identifier(ctxt, start, idx),
          None => return self.match_identifier(ctxt, start, self.buffer.len()),
        };
      },
      Some((start, ch)) if is_operator_start(ch) => loop {
        match self.iter.peek() {
          Some(&(_, ch)) if is_operator_continue(ch) => self.iter.next(),
          Some(&(idx, _)) => return self.match_operator(ctxt, start, idx),
          None => return self.match_operator(ctxt, start, self.buffer.len()),
        };
      },
      Some((start0, ch0)) if ch0.is_digit(10) => match self.iter.peek() {
        Some(&(_, ch1)) => {
          self.iter.next();

          if let (true, Some(base)) = (ch0 == '0', base_of_letter(ch1)) {
            match self.iter.next() {
              Some((start, ch)) if ch.is_digit(base) => {
                self.lex_number(start, base)
              }
              Some((_, ch)) => panic!(
                "Invalid integral literal with base {}; found {}",
                base, ch,
              ),
              None => panic!("Unexpected EOF"),
            }
          } else if is_ident_continue(ch1) {
            panic!("add whitespace after numbers, before identifier characters")
          } else {
            self.lex_number(start0, 10)
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
