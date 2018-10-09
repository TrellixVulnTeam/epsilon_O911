use crate::interner::Internable;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum IntSize {
  I8,
  I16,
  I32,
  I64,
  ISize,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mutability {
  Immutable,
  Mutable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Type<'tx> {
  SignedInt {
    size: IntSize,
  },
  UnsignedInt {
    size: IntSize,
  },

  Pointer {
    mutability: Mutability,
    pointee: &'tx Type<'tx>,
  },
}

impl<'tx> Internable for Type<'tx> {
  type Borrowed = Type<'tx>;
  type External = Type<'tx>;
  type Comparable = Type<'tx>;

  #[inline(always)]
  fn external_to_cmp<'a>(x: &'a Type<'tx>) -> &'a Type<'tx> {
    x
  }
  #[inline(always)]
  fn as_borrowed(&self) -> &Type<'tx> {
    self
  }
  #[inline(always)]
  fn from_external(x: &Type<'tx>) -> Self {
    *x
  }
}
