use crate::interner::{Interned, Interner};
use crate::{string, types};

use std::cell::UnsafeCell;

pub struct Context {
  identifiers: Interner<string::NfcStringBuf>,
  string_literals: UnsafeCell<Vec<String>>,
  // type_definitions: Vec<types::TypeDefinition>,
  types: Interner<types::Type<'static>>,
}

impl Context {
  pub fn new() -> Self {
    Self {
      identifiers: Interner::new(),
      string_literals: UnsafeCell::new(vec![]),
      types: Interner::new(),
    }
  }

  pub fn get_ident(&self, id: &str) -> Interned<string::NfcString> {
    self.identifiers.add_element(id)
  }

  pub fn get_string_literal(&self, str_lit: &str) -> &str {
    unsafe {
      let slit = &mut *self.string_literals.get();
      let string = str_lit.to_string();
      slit.push(string);
      let raw = (&*slit[slit.len() - 1]) as *const str;
      &*raw
    }
  }

  pub fn get_type<'cx>(
    &'cx self,
    ty: types::Type<'cx>,
  ) -> Interned<'cx, types::Type<'cx>> {
    let lt_erased = unsafe {
      std::mem::transmute::<types::Type<'cx>, types::Type<'static>>(ty)
    };

    self.types.add_element(&lt_erased)
  }
}
