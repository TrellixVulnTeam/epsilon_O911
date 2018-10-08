mod lexer;

use self::lexer::{Lexer, StringKind, Token};
use crate::string::NfcString;
use crate::IdentInterner;

pub struct Parser<'i, 's> {
  lexer: Lexer<'s>,
  intern: &'i IdentInterner,
  peek: Option<Token<'i>>,
}

#[derive(Debug)]
pub struct FunctionDecl<'i> {
  name: &'i NfcString,
  ret_ty: &'i NfcString,
}

#[derive(Debug)]
pub struct Function<'i> {
  decl: FunctionDecl<'i>,
  body: Expression<'i>,
}

#[derive(Debug)]
pub enum Item<'i> {
  ExternFunction(FunctionDecl<'i>),
  Function(Function<'i>),
}

#[derive(Debug)]
pub enum Expression<'i> {
  IntegerLiteral(u64),
  Name(&'i NfcString),
  StringLiteral(StringKind, &'i NfcString),
}

impl<'i, 's> Parser<'i, 's> {
  pub fn new(buffer: &'s str, intern: &'i IdentInterner) -> Self {
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

  fn get_ident(&mut self) -> &'i NfcString {
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
      Token::StringLiteral(kind, s) => Expression::StringLiteral(kind, s),
      tok => panic!("Expected expression, found {:?}", tok),
    }
  }

  fn parse_function_decl(&mut self) -> FunctionDecl<'i> {
    let name = self.get_ident();
    let () = self.eat_token(Token::OpenParen);
    let () = self.eat_token(Token::CloseParen);
    let () = self.eat_token(Token::Arrow);
    let ret_ty = self.get_ident();

    FunctionDecl { name, ret_ty }
  }

  fn parse_function(&mut self) -> Function<'i> {
    let decl = self.parse_function_decl();
    let () = self.eat_token(Token::OpenBrace);
    let body = self.parse_expression();
    let () = self.eat_token(Token::CloseBrace);

    Function { decl, body }
  }

  pub fn next_item(&mut self) -> Option<Item<'i>> {
    match self.next_token() {
      Token::KeywordExtern => {
        let () = self.eat_token(Token::KeywordFunc);
        let decl = self.parse_function_decl();
        let () = self.eat_token(Token::Semicolon);
        Some(Item::ExternFunction(decl))
      }
      Token::KeywordFunc => Some(Item::Function(self.parse_function())),
      Token::Eof => None,
      tok => panic!("unexpected token: {:?}", tok),
    }
  }
}
