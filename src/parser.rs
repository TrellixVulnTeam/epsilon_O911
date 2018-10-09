mod lexer;

use self::lexer::{Lexer, StringKind, Token};
use crate::context::Context;
use crate::interner::Interned;
use crate::string::NfcString;

#[derive(Debug)]
pub enum Type<'cx> {
  Named(Interned<'cx, NfcString>),
}

pub struct Parser<'cx, 's> {
  lexer: Lexer<'s>,
  ctxt: &'cx Context,
  peek: Option<Token<'cx>>,
}

#[derive(Debug)]
pub struct FunctionDecl<'cx> {
  name: Interned<'cx, NfcString>,
  ret_ty: Type<'cx>,
}

#[derive(Debug)]
pub struct Function<'cx> {
  decl: FunctionDecl<'cx>,
  body: Expression<'cx>,
}

#[derive(Debug)]
pub enum Item<'cx> {
  ExternFunction(FunctionDecl<'cx>),
  Function(Function<'cx>),
}

#[derive(Debug)]
pub enum Expression<'cx> {
  IntegerLiteral(u64),
  Name(Interned<'cx, NfcString>),
  StringLiteral(StringKind, &'cx str),
}

impl<'cx, 's> Parser<'cx, 's> {
  pub fn new(buffer: &'s str, ctxt: &'cx Context) -> Self {
    let lexer = Lexer::new(buffer);
    Parser {
      lexer,
      ctxt,
      peek: None,
    }
  }

  fn next_token(&mut self) -> Token<'cx> {
    let ret = match self.peek.take() {
      Some(tok) => tok,
      None => self.lexer.next_token(&self.ctxt),
    };

    //println!("{:?}", ret);

    ret
  }

  fn peek_token(&mut self) -> Token<'cx> {
    match self.peek {
      Some(tok) => tok,
      None => {
        let tok = self.lexer.next_token(&self.ctxt);
        self.peek = Some(tok);
        tok
      }
    }
  }

  fn get_ident(&mut self) -> Interned<'cx, NfcString> {
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

  fn parse_type(&mut self) -> Type<'cx> {
    match self.next_token() {
      Token::Identifier(s) => Type::Named(s),
      tok => panic!("Expected type, found {:?}", tok),
    }
  }

  fn parse_expression(&mut self) -> Expression<'cx> {
    match self.next_token() {
      Token::IntegerLiteral(i) => Expression::IntegerLiteral(i),
      Token::Identifier(s) => Expression::Name(s),
      Token::StringLiteral(kind, s) => Expression::StringLiteral(kind, s),
      tok => panic!("Expected expression, found {:?}", tok),
    }
  }

  fn parse_function_decl(&mut self) -> FunctionDecl<'cx> {
    let name = self.get_ident();
    let () = self.eat_token(Token::OpenParen);
    let () = self.eat_token(Token::CloseParen);
    let () = self.eat_token(Token::Arrow);
    let ret_ty = self.parse_type();

    FunctionDecl { name, ret_ty }
  }

  fn parse_function(&mut self) -> Function<'cx> {
    let decl = self.parse_function_decl();
    let () = self.eat_token(Token::OpenBrace);
    let body = self.parse_expression();
    let () = self.eat_token(Token::CloseBrace);

    Function { decl, body }
  }

  pub fn next_item(&mut self) -> Option<Item<'cx>> {
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
