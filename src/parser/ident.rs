use alloc::alloc;
use unicode_normalization::UnicodeNormalization;

pub struct Identifier<'a> {
  ptr: &'a IdentifierInner,
}

pub struct Context {
  set: ()//UnsafeCell<HashSet<IdentifierBox>>,
}

impl Context {
  pub fn new() -> Self {
    panic!()
  }

  pub fn add_ident(&self, s: &str) -> Identifier {
    panic!()
  }
}

// IdentifierBox's are _always_ unicode normalized by NFC
pub struct IdentifierBox {
  ptr: std::ptr::NonNull<IdentifierInner>,
}

impl IdentifierBox {
  // note: does unicode normalization
  crate fn new(s: &str) -> IdentifierBox {
    let size = s.nfc().map(|c| c.len_utf8()).sum();

    unsafe {
      assert!(size <= u32::max_value() as usize);
      let full_size = std::mem::size_of::<IdentifierInner>() + size;
      let align = std::mem::align_of::<IdentifierInner>();
      let layout = alloc::Layout::from_size_align_unchecked(full_size, align);
      let ptr = alloc::alloc(layout) as *mut IdentifierInner;

      std::ptr::write(&mut (*ptr).size, size as u32);

      let mut buff = (*ptr).mut_ptr();
      let mut remaining = size;
      for ch in s.nfc() {
        let offset = ch
          .encode_utf8(std::slice::from_raw_parts_mut(buff, remaining))
          .len();
        buff = buff.offset(offset as isize);
        remaining -= offset;
      }
      assert!(remaining == 0);

      IdentifierBox {
        ptr: std::ptr::NonNull::new_unchecked(ptr),
      }
    }
  }
}

impl Drop for IdentifierBox {
  fn drop(&mut self) {
    unsafe {
      let size = std::mem::size_of::<IdentifierInner>() + self.len();
      let align = std::mem::align_of::<IdentifierInner>();
      let layout = alloc::Layout::from_size_align_unchecked(size, align);
      alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
    }
  }
}

impl std::ops::Deref for IdentifierBox {
  type Target = IdentifierInner;

  fn deref(&self) -> &IdentifierInner {
    unsafe { self.ptr.as_ref() }
  }
}

#[repr(C)]
pub struct IdentifierInner {
  size: u32,
  array: [u8; 0], // pointer to the array
}

impl IdentifierInner {
  fn len(&self) -> usize {
    self.size as usize
  }
  fn ptr(&self) -> *const u8 {
    &self.array as *const [u8; 0] as *const u8
  }
  fn mut_ptr(&mut self) -> *mut u8 {
    &mut self.array as *mut [u8; 0] as *mut u8
  }

  crate fn as_str(&self) -> &str {
    unsafe {
      let utf8 = std::slice::from_raw_parts(self.ptr(), self.len());
      std::str::from_utf8(utf8).unwrap()
    }
  }
}
