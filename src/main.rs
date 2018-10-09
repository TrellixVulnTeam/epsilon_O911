mod interner;
#[allow(dead_code)]
mod llvm;
mod parser;
mod string;
mod types;

use crate::llvm::*;
use crate::parser::Parser;

const PROGRAM: &str = r#"
extern func puts() -> Int32;

func main() -> Int32 {
  0
}
"#;

mod ctxt {
  use crate::interner::Interner;
  use crate::{types, string};

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

    pub fn get_ident(&self, id: &str) -> &string::NfcString {
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

    pub fn get_type<'cx>(&'cx self, ty: types::Type<'cx>) -> &'cx types::Type<'cx> {
      let lt_erased = unsafe {
        std::mem::transmute::<types::Type<'cx>, types::Type<'static>>(ty)
      };

      self.types.add_element(&lt_erased)
    }
  }
}
type Context = ctxt::Context;

fn main() {
  let ctxt = Context::new();
  let mut parser = Parser::new(PROGRAM, &ctxt);

  while let Some(item) = parser.next_item() {
    println!("{:?}", item)
  }
}

#[allow(unused)]
fn test_llvm(ctxt: &Context) {
  let llctxt = llvm::Context::new();

  let int_ty = Type::int32(&llctxt);
  let char_ty = Type::int8(&llctxt);
  let pchar_ty = Type::ptr(char_ty);

  let puts_fun_ty = FunctionType::new(int_ty, &[pchar_ty]);
  let puts_fun = Function::new(ctxt.get_ident("puts"), puts_fun_ty, &llctxt);

  let main_fun_ty = FunctionType::new(int_ty, &[]);
  let main_fun = Function::new(ctxt.get_ident("main"), main_fun_ty, &llctxt);
  let initial_bb = main_fun.append_bb();

  let mut builder = Builder::new(&llctxt);
  builder.attach_to_bb(initial_bb);

  let hello_array = [
    ConstValue::int(char_ty, b'h', false),
    ConstValue::int(char_ty, b'e', false),
    ConstValue::int(char_ty, b'l', false),
    ConstValue::int(char_ty, b'l', false),
    ConstValue::int(char_ty, b'o', false),
    ConstValue::int(char_ty, b'\n', false),
    ConstValue::int(char_ty, 0u64, false),
  ];
  let hello = ConstValue::array(char_ty, &hello_array);
  let hello_glob = ConstValue::global(&llctxt, hello);

  let zero = ConstValue::int(Type::size_type(&llctxt), 0u64, false);
  let arg = ConstValue::gep(hello_glob, &[zero, zero]);

  builder.build_call(puts_fun, &[Value::from(arg)]);

  builder.build_ret(Value::from(ConstValue::int(int_ty, 0u64, false)));

  llctxt.dump();

  //let mut file = String::new();
  //llctxt.write_asm_file(&mut file).unwrap();
  //println!("{}", file);
}
