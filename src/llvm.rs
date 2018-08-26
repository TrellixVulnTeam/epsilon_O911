use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;

macro_rules! cstr {
  ($s:expr) => {
    concat!($s, "\0") as *const str as *const [u8] as *const u8 as *const libc::c_char
  };
}

#[allow(non_upper_case_globals)]
const LLVMFalse: LLVMBool = false as LLVMBool;
#[allow(non_upper_case_globals)]
const LLVMTrue: LLVMBool = true as LLVMBool;

pub struct Context {
  context: LLVMContextRef,
  triple: *mut libc::c_char,
  target: LLVMTargetRef,
  target_machine: LLVMTargetMachineRef,
  module: LLVMModuleRef,
}

impl Context {
  fn global_initialize() {
    unsafe {
      use lazy_static::lazy_static;
      lazy_static! {
        static ref INITIALIZED: std::sync::Mutex<bool> = std::sync::Mutex::new(false);
      }
      let mut initialized = INITIALIZED.lock().unwrap();
      if !*initialized {
        if LLVM_InitializeNativeTarget() != 0 {
          panic!("initialize native target");
        }
        if LLVM_InitializeNativeAsmPrinter() != 0 {
          panic!("initialize native asm printer");
        }
        *initialized = true;
      }
    }
  }

  pub fn new() -> Self {
    unsafe {
      Self::global_initialize();

      let opt_level = LLVMCodeGenOptLevel::LLVMCodeGenLevelNone;

      let context = LLVMContextCreate();

      let mut err: *mut libc::c_char = std::ptr::null_mut();

      let triple = LLVMGetDefaultTargetTriple();
      let mut target: LLVMTargetRef = std::mem::zeroed();
      if LLVMGetTargetFromTriple(triple, &mut target, &mut err) != 0 {
        panic!(
          "couldn't get target from triple: {}",
          std::ffi::CStr::from_ptr(err).to_str().unwrap()
        );
      }

      let target_machine = LLVMCreateTargetMachine(
        target,
        triple,
        cstr!(""),
        cstr!(""),
        opt_level,
        LLVMRelocMode::LLVMRelocDefault,
        LLVMCodeModel::LLVMCodeModelDefault,
      );

      let module = LLVMModuleCreateWithNameInContext(cstr!(""), context);

      Context {
        context,
        triple,
        target,
        target_machine,
        module,
      }
    }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeModule(self.module);
      LLVMDisposeTargetMachine(self.target_machine);
      LLVMDisposeMessage(self.triple);
      LLVMContextDispose(self.context);
    }
  }
}

pub fn hello_world(module: &mut Context) {
  unsafe {
    let builder = LLVMCreateBuilder();

    let void_ty = LLVMVoidType();
    let mut parameters = [];
    let void_fun_ty = LLVMFunctionType(
      void_ty,
      parameters.as_mut_ptr(),
      parameters.len() as libc::c_uint,
      LLVMFalse,
    );

    let hello_fun = LLVMAddFunction(module.module, cstr!("hello_world"), void_fun_ty);
    let initial_bb = LLVMAppendBasicBlock(hello_fun, cstr!("bb0"));
    LLVMPositionBuilderAtEnd(builder, initial_bb);

    LLVMBuildRetVoid(builder);

    LLVMDumpModule(module.module);
  }
}
