#[macro_use]
mod macros;

#[allow(dead_code)]
mod llvm;

mod parser;

//use self::llvm::*;

use self::parser::ident::IdentifierBox;

fn main() {
  let latin_capital_a_with_ring = r#"Å"#;
  let angstrom = r#"Å"#;
  let combining = r#"Å"#;

  assert!(latin_capital_a_with_ring != angstrom);
  assert!(angstrom != combining);
  assert!(combining != latin_capital_a_with_ring);

  let lcr = IdentifierBox::new(latin_capital_a_with_ring);
  let ang = IdentifierBox::new(angstrom);
  let cmb = IdentifierBox::new(combining);

  assert!(lcr.as_str() == ang.as_str());
  assert!(ang.as_str() == cmb.as_str());
  assert!(cmb.as_str() == lcr.as_str());

  println!("lcr: {}", lcr.as_str());
  println!("ang: {}", ang.as_str());
  println!("cmb: {}", cmb.as_str());
  /*
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

  let zero = ConstValue::int(Type::size_type(&ctxt), 0, false);
  let arg = ConstValue::gep(hello_glob, &[zero, zero]);

  builder.build_call(puts_fun, &[Value::from(arg)]);

  builder.build_ret(Value::from(ConstValue::int(int_ty, 0, false)));

  ctxt.dump();
  //let mut file = String::new();
  //ctxt.write_asm_file(&mut file).unwrap();
  //println!("{}", file);

  //llvm::output_to_file(&mut ctxt, "./hello-world.obj");
  */
}
