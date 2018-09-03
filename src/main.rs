#[allow(dead_code)]
mod llvm;

mod interner;

mod parser;

use crate::llvm::*;
use crate::parser::lexer::*;

fn main() {
  let intern = interner::Context::new();

  let mut lex = Lexer::new("func hello  hi", &intern);
  loop {
    match lex.next_token() {
      Token::Eof => break,
      tok => println!("{:?}", tok),
    }
  }

  let ctxt = llvm::Context::new();

  let int_ty = Type::int32(&ctxt);
  let char_ty = Type::int8(&ctxt);
  let pchar_ty = Type::ptr(char_ty);

  let puts_fun_ty = FunctionType::new(int_ty, &[pchar_ty]);
  let puts_fun = Function::new(intern.add_string("puts"), puts_fun_ty, &ctxt);

  let main_fun_ty = FunctionType::new(int_ty, &[]);
  let main_fun = Function::new(intern.add_string("main"), main_fun_ty, &ctxt);
  let initial_bb = main_fun.append_bb();

  let mut builder = Builder::new(&ctxt);
  builder.attach_to_bb(initial_bb);

  let hello_array = [
    ConstValue::int(char_ty, b'h' as _, false),
    ConstValue::int(char_ty, b'e' as _, false),
    ConstValue::int(char_ty, b'l' as _, false),
    ConstValue::int(char_ty, b'l' as _, false),
    ConstValue::int(char_ty, b'o' as _, false),
    ConstValue::int(char_ty, b'\n' as _, false),
    ConstValue::int(char_ty, 0, false),
  ];
  let hello = ConstValue::array(char_ty, &hello_array);
  let hello_glob = ConstValue::global(&ctxt, hello);

  let zero = ConstValue::int(Type::size_type(&ctxt), 0, false);
  let arg = ConstValue::gep(hello_glob, &[zero, zero]);

  builder.build_call(puts_fun, &[Value::from(arg)]);

  builder.build_ret(Value::from(ConstValue::int(int_ty, 0, false)));

  ctxt.dump();
  //let mut file = String::new();
  //ctxt.write_asm_file(&mut file).unwrap();
  //println!("{}", file);

  //llvm::output_to_file(&mut ctxt, "./hello-world.obj");
}
