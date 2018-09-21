mod lexer;

use self::lexer::{Lexer, Token};
use crate::interner::{Context as Interner, InternedString};

pub struct Parser<'i, 's> {
  lexer: Lexer<'s>,
  intern: &'i Interner,
  peek: Option<Token<'i>>,
}

#[derive(Debug)]
pub struct Function<'i> {
  name: InternedString<'i>,
  ret: InternedString<'i>,
  body: Expression<'i>,
}

#[derive(Debug)]
pub enum Item<'i> {
  Function(Function<'i>),
}

#[derive(Debug)]
pub enum Expression<'i> {
  IntegerLiteral(u64),
  Name(InternedString<'i>),
}

impl<'i, 's> Parser<'i, 's> {
  pub fn new(buffer: &'s str, intern: &'i Interner) -> Self {
    let lexer = Lexer::new(buffer);
    Parser {
      lexer,
      intern,
      peek: None,
    }
  }

  fn next_token(&mut self) -> Token<'i> {
    match self.peek.take() {
      Some(tok) => tok,
      None => self.lexer.next_token(&self.intern),
    }
  }

  fn peek_token(&mut self) -> Token<'i> {
    match self.peek {
      Some(tok) => tok,
      None => {
        let tok = self.lexer.next_token(&self.intern);
        self.peek = Some(tok);
        tok
      }
    }
  }

  pub fn parse_function(&mut self) -> Function<'i> {
    panic!("not yet implemented")
  }

  pub fn next_item(&mut self) -> Option<Item<'i>> {
    match self.next_token() {
      Token::KeywordFunc => Some(Item::Function(self.parse_function())),
      Token::Eof => None,
      tok => panic!("unexpected token: {:?}", tok)
    }
  }
}
