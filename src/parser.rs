mod lexer;

use self::lexer::{Lexer, Token};
use crate::interner::{Context as Interner, InternedString};

pub struct Parser<'i, 's> {
  lexer: Lexer<'s>,
  intern: &'i Interner,
}

pub enum Item<'i> {
  Function {
    name: InternedString<'i>,
    ret: InternedString<'i>,
    body: Block<'i>,
  },
}

pub enum Expression<'i> {
  Name(InternedString<'i>),
}

pub fn parse_test(s: &str, intern: &Interner) {
  let mut lex = Lexer::new(s);
  loop {
    match lex.next_token(intern) {
      Token::Eof => break,
      tok => println!("{:?}", tok),
    }
  }
}
