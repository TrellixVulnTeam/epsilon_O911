macro_rules! cstr {
  (rust $s:expr) => {
    unsafe {
      std::ffi::CStr::from_bytes_with_nul_unchecked(
        &*(concat!($s, "\0") as *const str as *const [u8]),
      )
    }
  };
  ($s:expr) => {
    concat!($s, "\0") as *const str as *const libc::c_char
  };
}

macro_rules! axiom {
  ($ax:expr ; $($reason:tt)*) => {
    if ! $ax {
      if cfg!(debug) {
        unreachable!()
      } else {
        std::hint::unreachable_unchecked();
      }
    }
  }
}