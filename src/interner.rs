use std::borrow::Borrow;
use std::cmp::Ord;
use std::{cell, collections, fmt, hash};

pub struct Interned<'a, T: ?Sized> {
  __ptr: &'a T,
}

impl<'a, T: ?Sized + fmt::Debug> fmt::Debug for Interned<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    self.__ptr.fmt(x)
  }
}
impl<'a, T: ?Sized + fmt::Display> fmt::Display for Interned<'a, T> {
  fn fmt(&self, x: &mut fmt::Formatter) -> fmt::Result {
    self.__ptr.fmt(x)
  }
}

impl<'a, T: ?Sized> Copy for Interned<'a, T> {}
impl<'a, T: ?Sized> Clone for Interned<'a, T> {
  fn clone(&self) -> Self { *self }
}

impl<'a, T: ?Sized> hash::Hash for Interned<'a, T> {
  fn hash<H: hash::Hasher>(&self, h: &mut H) {
    (self.__ptr as *const T).hash(h)
  }
}

impl<'a, T: ?Sized> Interned<'a, T> {
  fn new(__ptr: &'a T) -> Self {
    Interned { __ptr }
  }

  pub fn as_ref(p: Self) -> &'a T {
    p.__ptr
  }
}

impl<'a, T: 'a + ?Sized> std::ops::Deref for Interned<'a, T> {
  type Target = T;

  fn deref(&self) -> &T {
    self.__ptr
  }
}

impl<'a, T: 'a + ?Sized> PartialEq for Interned<'a, T> {
  fn eq(&self, other: &Self) -> bool {
    (self.__ptr as *const _) == (other.__ptr as *const _)
  }
}
impl<'a, T: 'a + ?Sized> Eq for Interned<'a, T> {}

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

impl<T> Interner<T>
where
  T: Internable,
{
  pub fn new() -> Self {
    Interner {
      set: cell::UnsafeCell::new(collections::BTreeSet::new()),
    }
  }

  pub fn add_element<'a>(
    &'a self,
    element: &T::External,
  ) -> Interned<'a, T::Borrowed> {
    unsafe {
      // safe because we don't allow anybody to get a reference to the innards
      // without an indirection
      // and because we never remove
      let name_cmp = T::external_to_cmp(element);
      let inner = &mut *self.set.get();
      if let Some(b) = inner.get(name_cmp) {
        let buf = &*(b as *const T);
        Interned::new(buf.as_borrowed())
      } else {
        inner.insert(T::from_external(element));
        // this seems unnecessary, but BTreeSet doesn't have a full interface
        // also, break the lifetime relation between inner and the ref
        let buf = &*(inner.get(name_cmp).unwrap() as *const T);
        Interned::new(buf.as_borrowed())
      }
    }
  }
}
