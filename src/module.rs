pub mod types;

use std::collections::HashMap;

use self::types::Type;

use crate::parser::{self, Parser};
use crate::interner::{Interner, Interned};
use crate::string::NfcString;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Expression<'cx> {
  IntegerLiteral(u64),
  StringLiteral(Interned<'cx, NfcString>)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration<'cx> {
  name: Interned<'cx, NfcString>,
  ret_ty: Interned<'cx, Type<'cx>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition<'cx> {
  declaration: FunctionDeclaration<'cx>,
  body: Expression<'cx>,
}

#[derive(Debug, Clone)]
pub enum Function<'cx> {
  External(FunctionDeclaration<'cx>),
  Definition(FunctionDefinition<'cx>),
}

pub struct Context<'cx> {
  pub parse_context: &'cx parser::Context,
  types: Interner<Type<'cx>>,
  type_names: UnsafeCell<HashMap<Interned<'cx, NfcString>, Interned<'cx, Type<'cx>>>>,
}

pub struct Module<'cx> {
  context: &'cx Context<'cx>,
  functions: HashMap<Interned<'cx, NfcString>, Function<'cx>>,
}

impl<'cx> Function<'cx> {
  pub fn declaration(&self) -> FunctionDeclaration<'cx> {
    match *self {
      Function::External(decl) => decl,
      Function::Definition(FunctionDefinition { declaration, .. }) => declaration,
    }
  }
}

impl<'cx> Type<'cx> {
  fn from_parse(ty: parser::Type<'cx>, ctxt: &'cx Context<'cx>) -> Self {
    match ty {
      parser::Type::Named(name) => match name.as_str() {
        "Int32" =>
        _ =>
      }
    }
  }
}

impl<'cx> Context<'cx> {
  pub fn new(parse_context: &'cx parser::Context) -> Self {
    Self {
      parse_context,
      types: Interner::new(),
    }
  }

  pub fn add_type_definition(&'cx self, ty: Type<'cx>) -> Interned<'cx, Type<'cx>> {
    let lt_erased = unsafe {
      std::mem::transmute::<Type<'cx>, Type<'static>>(ty)
    };

    self.types.add_element(&lt_erased)
  }

  pub fn add_named_type(&'cx self, name: Interned<'cx, NfcString>, ty: Interned<'cx, Type<'cx>>) {
    unsafe {
      let type_names = &mut *self.type_names.get();
      match type_names.insert(name, ty) {
        Some(_) => panic!("multiple types with the same name"),
        None => (),
      }
    }
  }

  pub fn get_type(&'cx self, name: Interned<'cx, NfcString>) -> Option<Interned<'cx, Type<'cx>>> {
    unsafe {
      let type_names = &mut *self.type_names.get();
      type_names.get(name)
    }
  }
}


impl<'cx> Module<'cx> {
  pub fn new(mut parser: Parser<'cx, '_>, context: &'cx Context<'cx>) -> Self {
    use parser::Item;
    let mut functions = HashMap::new();
    while let Some(item) = parser.next_item() {
      match item {
        Item::ExternFunction(decl) => {},
        Item::Function(func) => (),
      }
    }

    panic!()
  }
}