
use ::std::{fmt, slice, borrow, ops, mem, ptr};
use ::std::ffi::{OsStr, OsString};
use ::std::os::windows::ffi::{OsStrExt, OsStringExt};

use ::oleaut32::{SysAllocStringLen};
use ::winapi::{OLECHAR, BSTR};

use super::bstring::{BString};

/// Borrow as raw `BSTR`.
///
/// Use at the FFI boundary, you are responsible for ensuring the raw `BSTR` does not outlive its owner.
pub trait AsRawBStr {
	/// Returns the raw `BSTR` for this type.
	///
	/// Do not mutate or take ownership of the returned `BSTR`.
	fn as_raw(&self) -> BSTR;
}
impl AsRawBStr for BSTR {
	fn as_raw(&self) -> BSTR {
		*self
	}
}

/// Borrowed `BSTR`.
#[repr(C)]
pub struct BStr([OLECHAR]);
impl BStr {
	/// Creates a `&BStr` from `OLECHAR`s for internal use.
	///
	/// # Safety
	///
	/// All `BSTR` **must** have their length prefix stored as `u32` in the first two words and be nul terminated.
	pub unsafe fn new(s: &[OLECHAR]) -> &BStr {
		Self::assert_invariant(s);
		mem::transmute(s)
	}
	/// Converts `BSTR` into a `&BStr` borrow.
	///
	/// # Safety
	///
	/// While the memory contents are `debug_assert!`ed, the returned lifetime is unbounded.
	pub unsafe fn from_raw<'ub>(raw: BSTR) -> &'ub BStr {
		if raw.is_null() {
			// Null is semantically equivalent to empty string
			BStr::empty()
		}
		else {
			// Extract length prefix: number of bytes as `u32`
			let len = (*(raw.offset(-2) as *const u32) >> 1) as usize;
			// Wrangle it into a fat pointer Rust expects
			let s = slice::from_raw_parts(raw.offset(-2), len.wrapping_add(3));
			BStr::new(s)
		}
	}
	/// Returns the empty borrowed `BStr`.
	pub fn empty() -> &'static BStr {
		static EMPTY_BSTR_WORDS: [OLECHAR; 3] = [0, 0, 0];
		unsafe { Self::new(&EMPTY_BSTR_WORDS) }
	}
	/// Clones the string encoded as `OsString`.
	pub fn to_os_string(&self) -> OsString {
		OsString::from_wide(self.as_ref())
	}
	unsafe fn assert_invariant(s: &[OLECHAR]) {
		debug_assert_eq!(s.len() * 2 - 6, *(s.as_ptr() as *const u32) as usize);
		debug_assert_eq!(s.last(), Some(&0));
	}
}

/// Empty borrowed string as a null `BSTR`.
///
/// Null `BSTR` and zero length `BSTR` must be semantically equivalent
/// however [`&BStr`](struct.BStr.html) cannot be null, so this type fills the gap.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NullBStr;
impl From<NullBStr> for &'static BStr {
	fn from(_: NullBStr) -> &'static BStr {
		BStr::empty()
	}
}
impl AsRef<BStr> for NullBStr {
	fn as_ref(&self) -> &BStr {
		BStr::empty()
	}
}
impl ops::Deref for NullBStr {
	type Target = BStr;
	fn deref(&self) -> &BStr {
		BStr::empty()
	}
}
impl AsRawBStr for NullBStr {
	fn as_raw(&self) -> BSTR {
		ptr::null_mut()
	}
}

//----------------------------------------------------------------
// AsRef, Deref, ToOwned, AsRawBStr

impl AsRef<[OLECHAR]> for BStr {
	fn as_ref(&self) -> &[OLECHAR] {
		unsafe {
			let chars = &self.0;
			slice::from_raw_parts(chars.as_ptr().offset(2), chars.len().wrapping_sub(3))
		}
	}
}
impl ops::Deref for BStr {
	type Target = [OLECHAR];
	fn deref(&self) -> &[OLECHAR] {
		self.as_ref()
	}
}
impl AsRawBStr for BStr {
	fn as_raw(&self) -> BSTR {
		unsafe {
			Self::assert_invariant(&self.0);
			self.0.as_ptr().offset(2) as BSTR
		}
	}
}

//----------------------------------------------------------------
// Borrow, ToOwned

impl borrow::Borrow<BStr> for BString {
	fn borrow(&self) -> &BStr {
		self.as_ref()
	}
}
impl ToOwned for BStr {
	type Owned = BString;
	fn to_owned(&self) -> BString {
		unsafe {
			let raw = SysAllocStringLen(self.as_raw(), self.len() as u32);
			if raw.is_null() {
				panic!("out of memory");
			}
			BString::from_raw(raw)
		}
	}
}

//----------------------------------------------------------------
// Eq, PartialEq

impl Eq for BStr {}
impl PartialEq<BStr> for BStr {
	fn eq(&self, rhs: &BStr) -> bool {
		if self.len() == rhs.len() {
			self.as_ref() == rhs.as_ref()
		}
		else {
			false
		}
	}
}
impl<'a> PartialEq<&'a BStr> for BStr {
	fn eq(&self, rhs: &&'a BStr) -> bool {
		self.eq(*rhs)
	}
}
impl<T: ?Sized + AsRef<OsStr>> PartialEq<T> for BStr {
	fn eq(&self, rhs: &T) -> bool {
		let mut it = self.as_ref().iter();
		for ch in rhs.as_ref().encode_wide() {
			if let Some(ex) = it.next() {
				if *ex == ch {
					continue;
				}
			}
			return false;
		}
		it.next() == None
	}
}

//----------------------------------------------------------------
// Default

impl<'a> Default for &'a BStr {
	fn default() -> &'a BStr {
		BStr::empty()
	}
}

//----------------------------------------------------------------
// Debug

impl fmt::Debug for BStr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let s = OsString::from_wide(self.as_ref());
		s.fmt(f)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use ::std::{ptr, fmt};
	use super::*;
	use ::winapi::{OLECHAR, BSTR};

	#[test]
	fn units() {
		let hello_words = [10u16, 0,'H' as OLECHAR,'e' as OLECHAR,'l' as OLECHAR,'l' as OLECHAR,'o' as OLECHAR,0];
		let hello = unsafe { BStr::new(&hello_words) };

		assert_eq!(hello.len(), 5);
		assert_eq!(BStr::empty().is_empty(), true);

		unsafe {
			let raw = hello.as_raw();
			assert_eq!(*raw.offset(0), 'H' as OLECHAR);
			assert_eq!(*raw.offset(1), 'e' as OLECHAR);
			assert_eq!(*raw.offset(2), 'l' as OLECHAR);
			assert_eq!(*raw.offset(3), 'l' as OLECHAR);
			assert_eq!(*raw.offset(4), 'o' as OLECHAR);
			assert_eq!(*raw.offset(5), 0);
			assert_eq!(raw, hello_words[2..].as_ptr() as BSTR);

			assert_eq!(BStr::from_raw(raw), hello);
			assert_eq!(BStr::from_raw(ptr::null_mut()), BStr::empty());
		}

		assert_eq!(hello, hello);
		assert_eq!(hello, "Hello");
		assert!(hello != "Hell");
		assert!(hello != "Hello\0");

		assert_eq!(format!("{:?}", hello), "\"Hello\"");

		struct ByRef(BSTR);
		impl ByRef {
			fn try_as_ref(&self) -> Option<&BStr> {
				Some(unsafe { BStr::from_raw(self.0) })
			}
		}
		impl fmt::Debug for ByRef {
			fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
				self.try_as_ref().map(|v| v as &fmt::Debug);
				Ok(())
			}
		}
		let byref = ByRef(hello.as_raw());
		format!("{:?}", byref);
	}

/*	#[test]
	fn repr() {unsafe {
		let hello_words = ['H' as OLECHAR,'e' as OLECHAR,'l' as OLECHAR,'l' as OLECHAR,'o' as OLECHAR,0];
		let raw: = ::oleaut32::SysAllocString(&hello_words as *const _);
		let data: &[OLECHAR] = ::std::slice::from_raw_parts(raw:.offset(-2), 8);
		println!("{:?}", data);
	}}*/
}
