
use ::std::{fmt, borrow, ops, mem, ptr};
use ::std::ffi::{OsStr};
use ::std::os::windows::ffi::{OsStrExt};

use ::oleaut32::{SysAllocStringLen, SysFreeString};
use ::winapi::{BSTR, OLECHAR};

use super::bstr::{BStr, AsRawBStr};

/// System allocated `BSTR`.
///
/// This type can safely move ownership of the string in and out of COM FFI interfaces.
///
/// Mutation requires a round-trip through `OsString`.
///
/// # Examples
///
/// Create an empty string.
///
/// ```
/// # use ::bstr::BString;
/// let empty = BString::new();
/// assert_eq!(empty.len(), 0);
/// ```
///
/// Take ownership of a system allocated `BSTR`.
///
/// ```
/// # use ::bstr::BString;
/// let raw = ::std::ptr::null_mut();
/// unsafe {
/// 	let bstring = BString::from_raw(raw);
/// 	assert_eq!(bstring, "");
/// }
/// // Make sure to not use `raw` afterwards!
/// ```
///
/// Surrender ownership of the `BSTR` for FFI.
///
/// ```
/// # use ::bstr::BString;
/// let bstring = BString::from("Leaked BString");
/// let raw = bstring.into_raw();
/// // `raw` contains system allocated `BSTR` that won't be automatically freed.
/// ```
///
/// Encode anything `AsRef<OsStr>`.
///
/// ```
/// # use ::bstr::BString;
/// let bstring = BString::from("Hello BString");
/// assert_eq!(bstring, "Hello BString");
/// ```
///
/// Encode anything `IntoIterator<Item = OLECHAR>`.
/// Requires a clonable iterator as it needs to know the length before allocation.
///
/// ```
/// # use ::bstr::{BString};
/// use ::std::ffi::OsStr;
/// use ::std::os::windows::ffi::OsStrExt;
/// use ::bstr::IntoBString;
///
/// let bstring = OsStr::new("Encoded BString").encode_wide().into_bstring();
/// assert_eq!(bstring, "Encoded BString");
/// ```
///
/// Collect anything `Iterator<Item = OLECHAR>`.
/// Allocates an intermediary `Vec<OLECHAR>` because its length needs to be known beforehand.
///
/// ```
/// # use ::bstr::BString;
/// use ::std::ffi::OsStr;
/// use ::std::os::windows::ffi::OsStrExt;
///
/// let bstring: BString = OsStr::new("Collected BString").encode_wide().collect();
/// assert_eq!(bstring, "Collected BString");
/// ```
pub struct BString(BSTR);
impl BString {
	/// Creates an empty string.
	///
	/// Does not allocate system memory.
	pub fn new() -> BString {
		BString(ptr::null_mut())
	}
	/// Takes ownership of a system allocated `BSTR`.
	///
	/// Null is allowed, as it is semantically equivalent to the empty string.
	pub unsafe fn from_raw(raw: BSTR) -> BString {
		BString(raw)
	}
	/// Surrenders ownership of the `BSTR` for FFI.
	pub fn into_raw(self) -> BSTR {
		let raw = self.0;
		mem::forget(self);
		raw
	}
}
impl Clone for BString {
	fn clone(&self) -> BString {
		unsafe {
			let cch = self.len() as u32;// SysStringLen(self.0);
			let raw = SysAllocStringLen(self.0, cch);
			if raw.is_null() {
				panic!("out of memory");
			}
			BString::from_raw(raw)
		}
	}
}
impl Default for BString {
	fn default() -> BString {
		BString::new()
	}
}
impl Drop for BString {
	fn drop(&mut self) {
		unsafe { let _ = SysFreeString(self.0); }
	}
}

//----------------------------------------------------------------
// From, FromIterator

use ::std::iter::{FromIterator};

impl<T: AsRef<OsStr>> From<T> for BString {
	fn from(s: T) -> BString {
		s.as_ref().encode_wide().into_bstring()
	}
}
impl FromIterator<OLECHAR> for BString {
	fn from_iter<T: IntoIterator<Item = OLECHAR>>(ty: T) -> BString {
		let vec: Vec<_> = ty.into_iter().collect();
		if vec.len() == 0 {
			BString(ptr::null_mut())
		}
		else {
			unsafe {
				let raw = SysAllocStringLen(vec.as_ptr(), vec.len() as u32);
				if raw.is_null() {
					panic!("out of memory");
				}
				BString::from_raw(raw)
			}
		}
	}
}

/// Convert into `BString`.
pub trait IntoBString {
	fn into_bstring(self) -> BString;
}
impl<I: IntoIterator<Item = OLECHAR>> IntoBString for I where I::IntoIter: Clone {
	fn into_bstring(self) -> BString {
		let iter = self.into_iter();
		// Have to know the length beforehand...
		let cch = iter.clone().count();
		// Null for empty strings
		if cch == 0 {
			BString(ptr::null_mut())
		}
		else {
			unsafe {
				// Malloc the string
				let raw = SysAllocStringLen(ptr::null_mut(), cch as u32);
				if raw.is_null() {
					panic!("out of memory");
				}
				// Encode the string
				for (i, ch) in iter.enumerate() {
					*raw.offset(i as isize) = ch;
				}
				BString::from_raw(raw)
			}
		}
	}
}

//----------------------------------------------------------------
// AsRef, Deref, AsRawBStr

impl AsRef<BStr> for BString {
	fn as_ref(&self) -> &BStr {
		unsafe { BStr::from_raw(self.0) }
	}
}
impl ops::Deref for BString {
	type Target = BStr;
	fn deref(&self) -> &BStr {
		unsafe { BStr::from_raw(self.0) }
	}
}
impl AsRawBStr for BString {
	fn as_raw(&self) -> BSTR {
		self.0
	}
}

//----------------------------------------------------------------
// Borrow, BorrowMut

impl borrow::Borrow<BSTR> for BString {
	fn borrow(&self) -> &BSTR {
		&self.0
	}
}
// This would allow you to replace the `BSTR` with something arbitrary in safe code...
// impl borrow::BorrowMut<BSTR> for BString {
// 	fn borrow_mut(&mut self) -> &mut BSTR {
// 		&mut self.0
// 	}
// }

//----------------------------------------------------------------
// Eq, PartialEq

impl Eq for BString {}
impl PartialEq<BString> for BString {
	fn eq(&self, rhs: &BString) -> bool {
		(**self).eq(&**rhs)
	}
}
impl PartialEq<BStr> for BString {
	fn eq(&self, rhs: &BStr) -> bool {
		(**self).eq(rhs)
	}
}
impl<'a> PartialEq<&'a BStr> for BString {
	fn eq(&self, rhs: &&'a BStr) -> bool {
		(**self).eq(*rhs)
	}
}
impl<T: ?Sized + AsRef<OsStr>> PartialEq<T> for BString {
	fn eq(&self, rhs: &T) -> bool {
		(**self).eq(rhs)
	}
}

//----------------------------------------------------------------
// Debug

impl fmt::Debug for BString {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(&**self).fmt(f)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use ::std::borrow::{Borrow, ToOwned};
	use ::bstr::{BStr};
	use super::*;

	#[test]
	fn units() {
		BString::new().clone();
		let hello = BString::from("Hello, World!");
		assert_eq!(hello, "Hello, World!");
		let clone = hello.clone();
		assert_eq!(clone, hello);
		assert_eq!(clone, "Hello, World!");
		drop(clone);
		assert_eq!(hello, "Hello, World!");
		let borrowed: &BStr = hello.borrow();
		assert_eq!(borrowed.to_owned(), "Hello, World!");
	}
}
