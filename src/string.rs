use unicode_normalization::UnicodeNormalization;
use std::ffi;

use crate::interner::Internable;

mod impls;

impl Internable for NfcStringBuf {
  type Borrowed = NfcString;
  type External = str;
  type Comparable = NfcCmpStr;

  fn external_to_cmp(s: &str) -> &NfcCmpStr {
    NfcCmpStr::from_str(s)
  }
  fn as_borrowed(&self) -> &NfcString {
    self
  }
  fn from_external(s: &str) -> Self {
    Self::new(s)
  }
}

pub struct NfcStringBuf {
  ptr: std::ptr::NonNull<NfcString>,
}

impl NfcStringBuf {
  // note: does unicode normalization, and nul-termination
  pub fn new(s: &str) -> NfcStringBuf {
    /*
      we assert this because `nfc` is allowed to triple the size of
      the original string, at most, and we don't want our lengths to
      be greater than `i32::max_value()` in size
      if your identifiers are that long, you are doing something wrong
    */
    assert!(s.len() < i32::max_value() as usize / 4);
    let len: usize = s
      .nfc()
      .map(|c| {
        debug_assert!(c != '\0');
        c.len_utf8()
      })
      .sum();
    let size = len + 1;

    unsafe {
      let full_size = std::mem::size_of::<NfcString>() + size;
      let align = std::mem::align_of::<NfcString>();
      let layout =
        std::alloc::Layout::from_size_align_unchecked(full_size, align);
      let ptr = std::alloc::alloc(layout) as *mut NfcString;

      std::ptr::write(&mut (*ptr).size, len as u32);

      let mut buff = std::slice::from_raw_parts_mut((*ptr).mut_ptr(), size);
      for ch in s.nfc() {
        let offset = ch.encode_utf8(buff).len();
        buff = &mut buff[offset..]
      }
      assert!(buff.len() == 1);
      buff[0] = b'\0';

      NfcStringBuf {
        ptr: std::ptr::NonNull::new_unchecked(ptr),
      }
    }
  }
}

impl Drop for NfcStringBuf {
  fn drop(&mut self) {
    unsafe {
      let size = std::mem::size_of::<NfcString>() + self.len() + 1;
      let align = std::mem::align_of::<NfcString>();
      let layout = std::alloc::Layout::from_size_align_unchecked(size, align);
      std::alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
    }
  }
}

#[repr(C)]
pub struct NfcString {
  size: u32,
  /*
    dynamically sized array with size `size`
    note: _always_ normalized to NFC, and given a null terminator.
    This invariant is upheld by NfcStringBuf
  */
  array: [u8; 0],
}

impl NfcString {
  pub fn len(&self) -> usize {
    self.size as usize
  }
  pub fn ptr(&self) -> *const u8 {
    &self.array as *const [u8; 0] as *const u8
  }
  pub fn mut_ptr(&mut self) -> *mut u8 {
    &mut self.array as *mut [u8; 0] as *mut u8
  }

  pub fn as_str(&self) -> &str {
    unsafe {
      let utf8 = std::slice::from_raw_parts(self.ptr(), self.len());
      std::str::from_utf8_unchecked(utf8)
    }
  }

  pub fn as_cstr(&self) -> &ffi::CStr {
    unsafe {
      let utf8 = std::slice::from_raw_parts(self.ptr(), self.len() + 1);
      ffi::CStr::from_bytes_with_nul_unchecked(utf8)
    }
  }

  pub fn as_cstr_ptr(&self) -> *const std::os::raw::c_char {
    self.ptr() as *const _
  }
}

pub struct NfcCmpStr(str);

impl NfcCmpStr {
  pub fn from_str(s: &str) -> &Self {
    use std::mem;

    unsafe { mem::transmute::<&str, &Self>(s) }
  }
}