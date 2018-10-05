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
  ret_ty: InternedString<'i>,
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

  fn get_ident(&mut self) -> InternedString<'i> {
    match self.next_token() {
      Token::Identifier(s) => s,
      tok => panic!("Expected ident, found {:?}", tok),
    }
  }

  fn eat_token(&mut self, tok: Token) {
    let next_tok = self.next_token();
    if tok != next_tok {
      panic!("Expected {:?}, found {:?}", tok, next_tok)
    }
  }

  fn parse_expression(&mut self) -> Expression<'i> {
    match self.next_token() {
      Token::IntegerLiteral(i) => Expression::IntegerLiteral(i),
      Token::Identifier(s) => Expression::Name(s),
      tok => panic!("Expected expression, found {:?}", tok),
    }
  }

  fn parse_function(&mut self) -> Function<'i> {
    let name = self.get_ident();
    let () = self.eat_token(Token::OpenParen);
    let () = self.eat_token(Token::CloseParen);
    let () = self.eat_token(Token::Arrow);
    let ret_ty = self.get_ident();
    let () = self.eat_token(Token::OpenBrace);
    let body = self.parse_expression();
    let () = self.eat_token(Token::CloseBrace);

    Function { name, ret_ty, body }
  }

  pub fn next_item(&mut self) -> Option<Item<'i>> {
    match self.next_token() {
      Token::KeywordFunc => Some(Item::Function(self.parse_function())),
      Token::Eof => None,
      tok => panic!("unexpected token: {:?}", tok)
    }
  }
}
