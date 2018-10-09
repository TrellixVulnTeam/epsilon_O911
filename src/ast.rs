use crate::context::Context;
use crate::types::{Type, InternedType};

pub enum Expression<'cx> {
  IntegerLiteral(u64),
  StringLiteral(Interned<'cx, NfcString>)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FunctionDeclaration<'cx> {
  name: Interned<'cx, NfcString>,
  ret_ty: Interned<'cx, Type<'cx>>
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition<'cx> {
  declaration: FunctionDeclaration<'cx>
  body: Expression<'cx>,
}

#[derive(Debug, Clone)]
pub enum Function<'cx> {
  External(FunctionDeclaration<'cx>),
  Definition(FunctionDefinition<'cx>),
}

impl<'cx> Function<'cx> {
  pub fn declaration(&self) -> FunctionDeclaration<'cx> {
    use Function::*;
    match *self {
      External(decl) => decl,
      Definition(FunctionDefinition { declaration, .. }) => declaration,
    }
  }
}

pub struct Module<'cx> {
  context: &'cx Context,
  functions: HashMap<Interned<'cx, NfcString>, Function<'cx>>,
}

impl Module {
  pub fn new(ctxt: &'cx Context) -> Self {
    Module {
      context,
      functions: HashMap::new(),
    }
  }
}