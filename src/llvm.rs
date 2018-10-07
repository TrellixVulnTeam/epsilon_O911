use llvm_sys::core::*;
use llvm_sys::prelude::*;
use llvm_sys::target::*;
use llvm_sys::target_machine::*;
use llvm_sys::*;

use std::ffi::CStr;
use std::marker::PhantomData;

use crate::string::NfcString;

macro_rules! slice_to_llvm {
  ($underlying:ty) => {
    unsafe fn __slice_to_llvm_check_size(self) {
      std::mem::transmute::<Self, $underlying>(self);
    }

    fn slice_to_llvm(slice: &[Self]) -> (*mut $underlying, libc::c_uint) {
      debug_assert!(slice.len() <= libc::c_uint::max_value() as usize);
      (
        slice.as_ptr() as *mut Self as *mut $underlying,
        slice.len() as libc::c_uint,
      )
    }
  };
}
macro_rules! cstr {
  ($s:expr) => {
    concat!($s, "\0") as *const str as *const libc::c_char
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
  target_data: LLVMTargetDataRef,
  target_machine: LLVMTargetMachineRef,
  module: LLVMModuleRef,
}

impl Context {
  fn global_initialize() {
    unsafe {
      use lazy_static::lazy_static;
      lazy_static! {
        static ref INITIALIZED: std::sync::Mutex<bool> =
          std::sync::Mutex::new(false);
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

      let target_data = LLVMCreateTargetDataLayout(target_machine);

      let module = LLVMModuleCreateWithNameInContext(cstr!(""), context);

      Context {
        context,
        triple,
        target,
        target_data,
        target_machine,
        module,
      }
    }
  }

  pub fn pointer_size(&self) -> libc::c_uint {
    unsafe { LLVMPointerSize(self.target_data) }
  }

  pub fn dump(&self) {
    unsafe {
      LLVMDumpModule(self.module);
    }
  }

  pub fn write_asm_file<W: std::fmt::Write>(
    &self,
    w: &mut W,
  ) -> std::fmt::Result {
    unsafe {
      let mut buffer: LLVMMemoryBufferRef = std::ptr::null_mut();
      let mut err: *mut libc::c_char = std::ptr::null_mut();
      let failed = LLVMTargetMachineEmitToMemoryBuffer(
        self.target_machine,
        self.module,
        LLVMCodeGenFileType::LLVMAssemblyFile,
        &mut err,
        &mut buffer,
      );
      if failed != 0 {
        panic!(
          "Failed to emit object file: {}",
          CStr::from_ptr(err).to_str().unwrap()
        );
      }
      let ptr = LLVMGetBufferStart(buffer) as *mut u8;
      let len = LLVMGetBufferSize(buffer) as usize;
      let string =
        std::str::from_utf8(std::slice::from_raw_parts(ptr, len)).unwrap();
      w.write_str(string)
    }
  }

  pub fn write_obj_file<W: std::io::Write>(
    &self,
    w: &mut W,
  ) -> std::io::Result<()> {
    unsafe {
      let mut buffer: LLVMMemoryBufferRef = std::ptr::null_mut();
      let mut err: *mut libc::c_char = std::ptr::null_mut();
      let failed = LLVMTargetMachineEmitToMemoryBuffer(
        self.target_machine,
        self.module,
        LLVMCodeGenFileType::LLVMObjectFile,
        &mut err,
        &mut buffer,
      );
      if failed != 0 {
        panic!(
          "Failed to emit object file: {}",
          CStr::from_ptr(err).to_str().unwrap()
        );
      }
      let ptr = LLVMGetBufferStart(buffer) as *mut u8;
      let len = LLVMGetBufferSize(buffer) as usize;
      let slice = std::slice::from_raw_parts(ptr, len);
      w.write_all(slice)
    }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe {
      LLVMDisposeModule(self.module);
      LLVMDisposeTargetData(self.target_data);
      LLVMDisposeTargetMachine(self.target_machine);
      LLVMDisposeMessage(self.triple);
      LLVMContextDispose(self.context);
    }
  }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Type<'a> {
  ty: LLVMTypeRef,
  ctxt: PhantomData<&'a Context>,
}

impl<'a> Type<'a> {
  pub fn int8(ctxt: &'a Context) -> Self {
    unsafe {
      Type {
        ty: LLVMInt8TypeInContext(ctxt.context),
        ctxt: PhantomData,
      }
    }
  }
  pub fn int32(ctxt: &'a Context) -> Self {
    unsafe {
      Type {
        ty: LLVMInt32TypeInContext(ctxt.context),
        ctxt: PhantomData,
      }
    }
  }
  pub fn size_type(ctxt: &'a Context) -> Self {
    unsafe {
      Type {
        ty: LLVMIntTypeInContext(ctxt.context, ctxt.pointer_size() * 8),
        ctxt: PhantomData,
      }
    }
  }
  pub fn ptr(to: Type<'a>) -> Self {
    unsafe {
      Type {
        ty: LLVMPointerType(to.ty, 0),
        ctxt: to.ctxt,
      }
    }
  }

  slice_to_llvm!(LLVMTypeRef);
}

impl<'a> From<FunctionType<'a>> for Type<'a> {
  fn from(ty: FunctionType<'a>) -> Self {
    ty.0
  }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct FunctionType<'a>(Type<'a>);

impl<'a> FunctionType<'a> {
  pub fn new(ret_ty: Type<'a>, parms: &[Type<'a>]) -> Self {
    let (ptr, len) = Type::slice_to_llvm(parms);
    unsafe {
      FunctionType(Type {
        ty: LLVMFunctionType(ret_ty.ty, ptr, len, LLVMFalse),
        ctxt: PhantomData,
      })
    }
  }
  pub fn new_variadic(ret_ty: Type<'a>, parms: &[Type<'a>]) -> Self {
    let (ptr, len) = Type::slice_to_llvm(parms);
    unsafe {
      FunctionType(Type {
        ty: LLVMFunctionType(ret_ty.ty, ptr, len, LLVMTrue),
        ctxt: PhantomData,
      })
    }
  }

  unsafe fn from_type_unchecked(ty: Type<'a>) -> Self {
    FunctionType(ty)
  }
}

#[derive(Copy, Clone)]
pub struct Function<'a>(Value<'a>);

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct BasicBlock<'a> {
  bb: LLVMBasicBlockRef,
  ctxt: PhantomData<&'a Context>,
}

impl<'a> Function<'a> {
  pub fn new(
    name: &NfcString,
    ty: FunctionType<'a>,
    ctxt: &'a Context,
  ) -> Self {
    unsafe {
      Function(Value {
        value: LLVMAddFunction(ctxt.module, name.as_cstr_ptr(), ty.0.ty),
        ctxt: PhantomData,
      })
    }
  }

  pub fn append_bb(self) -> BasicBlock<'a> {
    unsafe {
      BasicBlock {
        bb: LLVMAppendBasicBlock(self.0.value, cstr!("")),
        ctxt: self.0.ctxt,
      }
    }
  }
}

#[repr(transparent)]
pub struct Builder<'a> {
  builder: LLVMBuilderRef,
  ctxt: PhantomData<&'a Context>,
}

impl<'a> Builder<'a> {
  pub fn new(ctxt: &'a Context) -> Self {
    unsafe {
      Builder {
        builder: LLVMCreateBuilderInContext(ctxt.context),
        ctxt: PhantomData,
      }
    }
  }

  pub fn attach_to_bb(&mut self, bb: BasicBlock<'a>) {
    unsafe {
      LLVMPositionBuilderAtEnd(self.builder, bb.bb);
    }
  }

  pub fn build_ret(&mut self, val: Value<'a>) {
    unsafe {
      LLVMBuildRet(self.builder, val.value);
    }
  }

  pub fn build_call(
    &mut self,
    fun: Function<'a>,
    args: &[Value<'a>],
  ) -> Value<'a> {
    let (ptr, len) = Value::slice_to_llvm(args);
    unsafe {
      Value {
        value: LLVMBuildCall(self.builder, fun.0.value, ptr, len, cstr!("")),
        ctxt: self.ctxt,
      }
    }
  }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Value<'a> {
  value: LLVMValueRef,
  ctxt: PhantomData<&'a Context>,
}

impl<'a> Value<'a> {
  slice_to_llvm!(LLVMValueRef);
}

impl<'a> From<ConstValue<'a>> for Value<'a> {
  fn from(cnst: ConstValue<'a>) -> Self {
    cnst.0
  }
}
impl<'a> From<Function<'a>> for Value<'a> {
  fn from(val: Function<'a>) -> Self {
    val.0
  }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ConstValue<'a>(Value<'a>);

impl<'a> ConstValue<'a> {
  pub fn int(ty: Type<'a>, value: u64, sign_ext: bool) -> Self {
    unsafe {
      ConstValue(Value {
        value: LLVMConstInt(ty.ty, value, sign_ext as LLVMBool),
        ctxt: PhantomData,
      })
    }
  }

  pub fn gep(from: ConstValue<'a>, indices: &[ConstValue<'a>]) -> Self {
    let (ptr, len) = Self::slice_to_llvm(indices);
    unsafe {
      ConstValue(Value {
        value: LLVMConstInBoundsGEP(from.0.value, ptr, len),
        ctxt: from.0.ctxt,
      })
    }
  }

  slice_to_llvm!(LLVMValueRef);

  pub fn array(ty: Type<'a>, arr: &[ConstValue<'a>]) -> Self {
    unsafe {
      let (ptr, len) = Self::slice_to_llvm(arr);
      ConstValue(Value {
        value: LLVMConstArray(ty.ty, ptr, len),
        ctxt: ty.ctxt,
      })
    }
  }

  pub fn global(ctxt: &'a Context, init: ConstValue<'a>) -> Self {
    unsafe {
      let glob_ptr =
        LLVMAddGlobal(ctxt.module, LLVMTypeOf(init.0.value), cstr!(""));
      LLVMSetInitializer(glob_ptr, init.0.value);
      LLVMSetGlobalConstant(glob_ptr, LLVMTrue);
      LLVMSetUnnamedAddr(glob_ptr, LLVMTrue);
      LLVMSetLinkage(glob_ptr, LLVMLinkage::LLVMPrivateLinkage);

      ConstValue(Value {
        value: glob_ptr,
        ctxt: init.0.ctxt,
      })
    }
  }
}
