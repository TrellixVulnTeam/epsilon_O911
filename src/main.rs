mod interner;
#[allow(dead_code)]
mod llvm;
mod parser;
mod string;

use crate::llvm::*;
use crate::parser::Parser;

const PROGRAM: &str = r#"
extern func puts() -> i32;

func main() -> i32 {
  0
}
"#;

type IdentInterner = interner::Interner<string::NfcStringBuf>;

fn main() {
  let intern = IdentInterner::new();
  let mut parser = Parser::new(PROGRAM, &intern);

  while let Some(item) = parser.next_item() {
    println!("{:?}", item)
  }
}

#[allow(unused)]
fn test_llvm(intern: &IdentInterner) {
  let ctxt = llvm::Context::new();

  let int_ty = Type::int32(&ctxt);
  let char_ty = Type::int8(&ctxt);
  let pchar_ty = Type::ptr(char_ty);

  let puts_fun_ty = FunctionType::new(int_ty, &[pchar_ty]);
  let puts_fun = Function::new(intern.add_element("puts"), puts_fun_ty, &ctxt);

  let main_fun_ty = FunctionType::new(int_ty, &[]);
  let main_fun = Function::new(intern.add_element("main"), main_fun_ty, &ctxt);
  let initial_bb = main_fun.append_bb();

  let mut builder = Builder::new(&ctxt);
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
  let hello_glob = ConstValue::global(&ctxt, hello);

  let zero = ConstValue::int(Type::size_type(&ctxt), 0u64, false);
  let arg = ConstValue::gep(hello_glob, &[zero, zero]);

  builder.build_call(puts_fun, &[Value::from(arg)]);

  builder.build_ret(Value::from(ConstValue::int(int_ty, 0u64, false)));

  ctxt.dump();

  //let mut file = String::new();
  //ctxt.write_asm_file(&mut file).unwrap();
  //println!("{}", file);
}
