#[macro_use]
#[allow(dead_code)]
mod llvm;

use self::llvm::*;

fn main() {
  let ctxt = llvm::Context::new();

  let int_ty = Type::int32(&ctxt);
  let char_ty = Type::int8(&ctxt);
  let pchar_ty = Type::ptr(char_ty);

  let puts_fun_ty = FunctionType::new(int_ty, &[pchar_ty]);
  let puts_fun = Function::new(cstr!(rust "puts"), puts_fun_ty, &ctxt);

  let main_fun_ty = FunctionType::new(int_ty, &[]);
  let main_fun = Function::new(cstr!(rust "main"), main_fun_ty, &ctxt);
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

  let zero = ConstValue::int(int_ty, 0, false);
  let arg = ConstValue::gep(hello_glob, &[zero, zero]);

  builder.build_call(puts_fun, &[Value::from(arg)]);

  builder.build_ret(Value::from(ConstValue::int(int_ty, 0, false)));

  ctxt.dump();

  //llvm::output_to_file(&mut ctxt, "./hello-world.obj");
}
