use alloc::alloc;
use std::{cell, collections, ffi};
use unicode_normalization::UnicodeNormalization;

mod impls;

#[derive(Copy, Clone)]
pub struct InternedString<'a> {
  ptr: &'a InternedStringInner,
}

impl<'a> InternedString<'a> {
  pub fn len(self) -> usize {
    self.ptr.len()
  }

  pub fn as_str(self) -> &'a str {
    self.ptr.as_str()
  }
  pub fn as_cstr(self) -> &'a ffi::CStr {
    self.ptr.as_cstr()
  }
  pub fn as_cstr_ptr(self) -> *const libc::c_char {
    self.ptr.ptr() as *const libc::c_char
  }
}

pub struct Context {
  // for safety, this must be append-only
  // _never_ remove any values from it
  // we use BTreeMap for the entry API
  set: cell::UnsafeCell<collections::BTreeSet<InternedStringBox>>,
}

impl Context {
  pub fn new() -> Self {
    Context {
      set: cell::UnsafeCell::new(collections::BTreeSet::new()),
    }
  }

  pub fn add_string<'a>(&'a self, name: &str) -> InternedString<'a> {
    unsafe {
      // safe because we don't allow anybody to get a reference to the innards
      // without an indirection
      // and because we never remove
      let name_cmp = NfcCmpStr::from_str(name);
      let inner = &mut *self.set.get();
      if let Some(b) = inner.get(name_cmp) {
        let ptr: &'a InternedStringInner = &*b.as_raw_ptr();
        InternedString { ptr }
      } else {
        inner.insert(InternedStringBox::new(name));
        // this seems unnecessary, but BTreeSet doesn't have a full interface
        let ptr: &'a InternedStringInner =
          &*inner.get(name_cmp).unwrap().as_raw_ptr();
        InternedString { ptr }
      }
    }
  }
}

struct InternedStringBox {
  ptr: std::ptr::NonNull<InternedStringInner>,
}

impl InternedStringBox {
  // note: does unicode normalization, and nul-termination
  fn new(s: &str) -> InternedStringBox {
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
      }).sum();
    let size = len + 1;

    unsafe {
      let full_size = std::mem::size_of::<InternedStringInner>() + size;
      let align = std::mem::align_of::<InternedStringInner>();
      let layout = alloc::Layout::from_size_align_unchecked(full_size, align);
      let ptr = alloc::alloc(layout) as *mut InternedStringInner;

      std::ptr::write(&mut (*ptr).size, len as u32);

      let mut buff = std::slice::from_raw_parts_mut((*ptr).mut_ptr(), size);
      for ch in s.nfc() {
        let offset = ch.encode_utf8(buff).len();
        buff = &mut buff[offset..]
      }
      assert!(buff.len() == 1);
      buff[0] = b'\0';

      InternedStringBox {
        ptr: std::ptr::NonNull::new_unchecked(ptr),
      }
    }
  }

  // for breaking lifetimes
  fn as_raw_ptr(&self) -> *const InternedStringInner {
    &**self
  }
}

impl Drop for InternedStringBox {
  fn drop(&mut self) {
    unsafe {
      let size = std::mem::size_of::<InternedStringInner>() + self.len() + 1;
      let align = std::mem::align_of::<InternedStringInner>();
      let layout = alloc::Layout::from_size_align_unchecked(size, align);
      alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
    }
  }
}

#[repr(C)]
struct InternedStringInner {
  size: u32,
  /*
    dynamically sized array with size `size`
    note: _always_ normalized to NFC, and given a null terminator.
    This invariant is upheld by InternedStringBox
  */
  array: [u8; 0],
}

impl InternedStringInner {
  fn len(&self) -> usize {
    self.size as usize
  }
  fn ptr(&self) -> *const u8 {
    &self.array as *const [u8; 0] as *const u8
  }
  fn mut_ptr(&mut self) -> *mut u8 {
    &mut self.array as *mut [u8; 0] as *mut u8
  }

  fn as_str(&self) -> &str {
    unsafe {
      let utf8 = std::slice::from_raw_parts(self.ptr(), self.len());
      std::str::from_utf8_unchecked(utf8)
    }
  }

  fn as_cstr(&self) -> &ffi::CStr {
    unsafe {
      let utf8 = std::slice::from_raw_parts(self.ptr(), self.len() + 1);
      ffi::CStr::from_bytes_with_nul_unchecked(utf8)
    }
  }
}

struct NfcCmpStr(str);

impl NfcCmpStr {
  fn from_str(s: &str) -> &Self {
    use std::mem;

    unsafe { mem::transmute::<&str, &Self>(s) }
  }
}
