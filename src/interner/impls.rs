use std::{borrow, cmp, fmt, ops};

use unicode_normalization::UnicodeNormalization;

use super::*;

// FORMATTING TRAIT IMPLS
impl<'a> fmt::Debug for NfcStringRef<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <str as fmt::Debug>::fmt(self.as_str(), f)
  }
}
impl<'a> fmt::Display for NfcStringRef<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    <str as fmt::Display>::fmt(self.as_str(), f)
  }
}

// DEREF TRAIT IMPLS
impl borrow::Borrow<NfcCmpStr> for NfcStringInner {
  fn borrow(&self) -> &NfcCmpStr {
    NfcCmpStr::from_str(self.as_str())
  }
}

impl ops::Deref for NfcStringBuf {
  type Target = NfcStringInner;

  fn deref(&self) -> &NfcStringInner {
    unsafe { self.ptr.as_ref() }
  }
}

impl borrow::Borrow<NfcStringInner> for NfcStringBuf {
  fn borrow(&self) -> &NfcStringInner {
    &**self
  }
}
impl borrow::Borrow<NfcCmpStr> for NfcStringBuf {
  fn borrow(&self) -> &NfcCmpStr {
    NfcCmpStr::from_str(self.as_str())
  }
}

// ORDERING TRAIT IMPLS

impl<'a> cmp::PartialEq for NfcStringRef<'a> {
  fn eq(&self, other: &Self) -> bool {
    self.ptr as *const _ == other.ptr as *const _
  }
}
impl<'a> cmp::Eq for NfcStringRef<'a> {}

impl<'a> cmp::PartialEq<str> for NfcStringRef<'a> {
  fn eq(&self, other: &str) -> bool {
    self.ptr.eq(other)
  }
}
impl<'a> cmp::PartialEq<NfcStringRef<'a>> for str {
  fn eq(&self, other: &NfcStringRef<'a>) -> bool {
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

impl cmp::PartialEq for NfcStringInner {
  fn eq(&self, other: &Self) -> bool {
    self.as_str() == other.as_str()
  }
}
impl cmp::Eq for NfcStringInner {}

impl cmp::PartialOrd for NfcStringInner {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    Some(self.cmp(other))
  }
}
impl cmp::Ord for NfcStringInner {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.as_str().cmp(other.as_str())
  }
}

impl cmp::PartialEq<str> for NfcStringInner {
  fn eq(&self, other: &str) -> bool {
    self.as_str().chars().eq(other.nfc())
  }
}
impl cmp::PartialEq<NfcCmpStr> for NfcStringInner {
  fn eq(&self, other: &NfcCmpStr) -> bool {
    self.as_str().chars().eq(other.0.nfc())
  }
}
impl cmp::PartialEq<NfcStringInner> for str {
  fn eq(&self, other: &NfcStringInner) -> bool {
    *other == *self
  }
}
impl cmp::PartialEq<NfcStringInner> for NfcCmpStr {
  fn eq(&self, other: &NfcStringInner) -> bool {
    *other == *self
  }
}

impl cmp::PartialOrd<str> for NfcStringInner {
  fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
    Some(self.as_str().chars().cmp(other.nfc()))
  }
}
impl cmp::PartialOrd<NfcStringInner> for str {
  fn partial_cmp(&self, other: &NfcStringInner) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<NfcCmpStr> for NfcStringInner {
  fn partial_cmp(&self, other: &NfcCmpStr) -> Option<cmp::Ordering> {
    Some(self.as_str().chars().cmp(other.0.nfc()))
  }
}
impl cmp::PartialOrd<NfcStringInner> for NfcCmpStr {
  fn partial_cmp(&self, other: &NfcStringInner) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}

impl cmp::PartialEq for NfcStringBuf {
  fn eq(&self, other: &Self) -> bool {
    (**self).eq(other)
  }
}
impl cmp::Eq for NfcStringBuf {}

impl cmp::PartialOrd for NfcStringBuf {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::Ord for NfcStringBuf {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    (**self).cmp(&**other)
  }
}

impl cmp::PartialEq<NfcStringInner> for NfcStringBuf {
  fn eq(&self, other: &NfcStringInner) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<NfcStringBuf> for NfcStringInner {
  fn eq(&self, other: &NfcStringBuf) -> bool {
    other.eq(self)
  }
}
impl cmp::PartialEq<str> for NfcStringBuf {
  fn eq(&self, other: &str) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<NfcStringBuf> for str {
  fn eq(&self, other: &NfcStringBuf) -> bool {
    other.eq(self)
  }
}
impl cmp::PartialEq<NfcCmpStr> for NfcStringBuf {
  fn eq(&self, other: &NfcCmpStr) -> bool {
    (**self).eq(other)
  }
}
impl cmp::PartialEq<NfcStringBuf> for NfcCmpStr {
  fn eq(&self, other: &NfcStringBuf) -> bool {
    other.eq(self)
  }
}

impl cmp::PartialOrd<NfcStringInner> for NfcStringBuf {
  fn partial_cmp(&self, other: &NfcStringInner) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<NfcStringBuf> for NfcStringInner {
  fn partial_cmp(&self, other: &NfcStringBuf) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<str> for NfcStringBuf {
  fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<NfcStringBuf> for str {
  fn partial_cmp(&self, other: &NfcStringBuf) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
impl cmp::PartialOrd<NfcCmpStr> for NfcStringBuf {
  fn partial_cmp(&self, other: &NfcCmpStr) -> Option<cmp::Ordering> {
    (**self).partial_cmp(other)
  }
}
impl cmp::PartialOrd<NfcStringBuf> for NfcCmpStr {
  fn partial_cmp(&self, other: &NfcStringBuf) -> Option<cmp::Ordering> {
    other.partial_cmp(self)
  }
}
