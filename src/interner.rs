use std::{cell, collections};
use std::borrow::Borrow;
use std::cmp::{Ord};

pub trait Internable: Ord + Borrow<<Self as Internable>::Comparable> {
  type Borrowed: ?Sized;
  type External: ?Sized;
  type Comparable: ?Sized + Ord;

  fn external_to_cmp(_: &Self::External) -> &Self::Comparable;
  fn as_borrowed(&self) -> &Self::Borrowed;
  fn from_external(_: &Self::External) -> Self;
}

pub struct Interner<T> {
  // for safety, this must be append-only
  // _never_ remove any values from it
  set: cell::UnsafeCell<collections::BTreeSet<T>>,
}

impl<T> Interner<T> where T: Internable {
  pub fn new() -> Self {
    Interner {
      set: cell::UnsafeCell::new(collections::BTreeSet::new()),
    }
  }

  pub fn add_element<'a>(&'a self, element: &T::External) -> &'a T::Borrowed {
    unsafe {
      // safe because we don't allow anybody to get a reference to the innards
      // without an indirection
      // and because we never remove
      let name_cmp = T::external_to_cmp(element);
      let inner = &mut *self.set.get();
      if let Some(b) = inner.get(name_cmp) {
        let buf = &*(b as *const T);
        buf.as_borrowed()
      } else {
        inner.insert(T::from_external(element));
        // this seems unnecessary, but BTreeSet doesn't have a full interface
        // also, break the lifetime relation between inner and the ref
        let buf = &*(inner.get(name_cmp).unwrap() as *const T);
        buf.as_borrowed()
      }
    }
  }
}
