#[allow(dead_code)]
mod llvm;

fn main() {
  let mut ctxt = llvm::Context::new();
  llvm::hello_world(&mut ctxt);
}
