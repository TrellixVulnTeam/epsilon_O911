use std::{borrow, cmp, fmt, ops};

use unicode_normalization::UnicodeNormalization;

use super::*;

// FORMATTING TRAIT IMPLS
impl<'a> fmt::Debug for InternedString<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <str as fmt::Debug>::fmt(self.as_str(), f)
  }
}
impl<'a> fmt::Display for InternedString<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <str as fmt::Display>::fmt(self.as_str(), f)
  }
}

// DEREF TRAIT IMPLS
impl borrow::Borrow<NfcCmpStr> for InternedStringInner {
  fn borrow(&self) -> &NfcCmpStr {
    NfcCmpStr::from_str(self.as_str())
  }
}

impl ops::Deref for InternedStringBox {
  type Target = InternedStringInner;

  fn deref(&self) -> &InternedStringInner {
    unsafe { self.ptr.as_ref() }
  }
}

impl borrow::Borrow<InternedStringInner> for InternedStringBox {
  fn borrow(&self) -> &InternedStringInner {
    &**self
  }
}
impl borrow::Borrow<NfcCmpStr> for InternedStringBox {
  fn borrow(&self) -> &NfcCmpStr {
    NfcCmpStr::from_str(self.as_str())
  }
}

// ORDERING TRAIT IMPLS

impl<'a> cmp::PartialEq for InternedString<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.ptr as *const _ == other.ptr as *const _
  }
}
impl<'a> cmp::Eq for InternedString<'a> {}

impl<'a> cmp::PartialEq<str> for InternedString<'a> {
  fn eq(&self, other: &str) -> bool {
    self.ptr.eq(other)
  }
}
impl<'a> cmp::PartialEq<InternedString<'a>> for str {
  fn eq(&self, other: &InternedString<'a>) -> bool {
    other.eq(self)
  }
}

impl cmp::PartialEq for NfcCmpStr {
  fn eq(&self, other: &Self) -> bool {
    self.0.nfc().eq(other.0.nfc())
  }
}
impl cmp::Eq for NfcCmpStr {}

impl cmp::PartialOrd for NfcCmpStr {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.0.nfc().cmp(other.0.nfc()))
  }
}
impl cmp::Ord for NfcCmpStr {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.0.nfc().cmp(other.0.nfc())
  }
}

impl cmp::PartialEq for InternedStringInner {
  fn eq(&self, other: &Self) -> bool {
    self.as_str() == other.as_str()
  }
}
impl cmp::Eq for InternedStringInner {}

impl cmp::PartialOrd for InternedStringInner {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl cmp::Ord for InternedStringInner {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.as_str().cmp(other.as_str())
  }
}

impl cmp::PartialEq<str> for InternedStringInner {
  fn eq(&self, other: &str) -> bool {
    self.as_str().chars().eq(other.nfc())
  }
}
impl cmp::PartialEq<NfcCmpStr> for InternedStringInner {
  fn eq(&self, other: &NfcCmpStr) -> bool {
    self.as_str().chars().eq(other.0.nfc())
  }
}
impl cmp::PartialEq<InternedStringInner> for str {
  fn eq(&self, other: &InternedStringInner) -> bool {
    *other == *self
  }
}
impl cmp::PartialEq<InternedStringInner> for NfcCmpStr {
  fn eq(&self, other: &InternedStringInner) -> bool {
    *other == *self
  }
}

impl cmp::PartialOrd<str> for InternedStringInner {
  fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
    Some(self.as_str().chars().cmp(other.nfc()))
  }
}
impl cmp::PartialOrd<InternedStringInner> for str {
  fn partial_cmp(&self, other: &InternedStringInner) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<NfcCmpStr> for InternedStringInner {
  fn partial_cmp(&self, other: &NfcCmpStr) -> Option<cmp::Ordering> {
    Some(self.as_str().chars().cmp(other.0.nfc()))
  }
}
impl cmp::PartialOrd<InternedStringInner> for NfcCmpStr {
  fn partial_cmp(&self, other: &InternedStringInner) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}

impl cmp::PartialEq for InternedStringBox {
  fn eq(&self, other: &Self) -> bool {
    (**self).eq(other)
  }
}
impl cmp::Eq for InternedStringBox {}

impl cmp::PartialOrd for InternedStringBox {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::Ord for InternedStringBox {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    (**self).cmp(&**other)
  }
}

impl cmp::PartialEq<InternedStringInner> for InternedStringBox {
  fn eq(&self, other: &InternedStringInner) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<InternedStringBox> for InternedStringInner {
  fn eq(&self, other: &InternedStringBox) -> bool {
    other.eq(self)
  }
}
impl cmp::PartialEq<str> for InternedStringBox {
  fn eq(&self, other: &str) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<InternedStringBox> for str {
  fn eq(&self, other: &InternedStringBox) -> bool {
    other.eq(self)
  }
}
impl cmp::PartialEq<NfcCmpStr> for InternedStringBox {
  fn eq(&self, other: &NfcCmpStr) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<InternedStringBox> for NfcCmpStr {
  fn eq(&self, other: &InternedStringBox) -> bool {
    other.eq(self)
  }
}

impl cmp::PartialOrd<InternedStringInner> for InternedStringBox {
  fn partial_cmp(&self, other: &InternedStringInner) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<InternedStringBox> for InternedStringInner {
  fn partial_cmp(&self, other: &InternedStringBox) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<str> for InternedStringBox {
  fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<InternedStringBox> for str {
  fn partial_cmp(&self, other: &InternedStringBox) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<NfcCmpStr> for InternedStringBox {
  fn partial_cmp(&self, other: &NfcCmpStr) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<InternedStringBox> for NfcCmpStr {
  fn partial_cmp(&self, other: &InternedStringBox) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
