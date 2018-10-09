mod context;
mod interner;
#[allow(dead_code)]
mod llvm;
mod parser;
mod string;
mod types;

use crate::llvm::*;
use crate::parser::Parser;

const PROGRAM: &str = r#"
extern func ccosf([% _: FloatComplex %]) -> FloatComplex;

[%
type FloatComplex = struct {
  x: Float32;
  y: Float32;
};
%]

func main() -> Int32 {
  0
}
"#;

fn main() {
  let ctxt = context::Context::new();
  let mut parser = Parser::new(PROGRAM, &ctxt);

  while let Some(item) = parser.next_item() {
    println!("{:?}", item)
  }
}

#[allow(unused)]
fn test_llvm(ctxt: &context::Context) {
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
