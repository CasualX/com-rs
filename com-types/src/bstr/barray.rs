
use ::std::{fmt, ops, slice, mem};
use ::std::iter::{FromIterator};
use ::std::ffi::{OsStr};
use ::std::os::windows::ffi::{OsStrExt};

use ::winapi::{BSTR, OLECHAR};

use super::array_traits::{Array};
use super::bstr::{BStr, AsRawBStr};

/// Array backed `BSTR`.
///
/// Enables stack allocated bstr literals, albeit with ugly syntax.
///
/// No `Cow`-like behaviour as it's meant for small bstr literals, simply panics if the string being stored doesn't fit in the backing array.
///
/// The number of ‘characters’ is limited to array capacity - 3.
///
/// # Examples
///
/// Create an empty literal.
///
/// ```
/// # use com_types::bstr::BArray;
/// let empty = BArray::<[_; 8]>::new();
/// assert!(empty.is_empty());
/// ```
///
/// Create a bstr literal.
///
/// ```
/// # use com_types::bstr::BArray;
/// let lit = BArray::<[_; 16]>::from("BStr Literal");
/// assert_eq!(lit.as_ref(), "BStr Literal");
/// ```
#[repr(C)]
pub struct BArray<A>(A) where A: Array<OLECHAR>;
impl<A: Array<OLECHAR>> BArray<A> {
	/// Creates an empty string.
	pub fn new() -> BArray<A> {
		unsafe {
			let mut arr: A = mem::uninitialized();
			{ let sl = arr.as_mut_slice(); sl[0] = 0; sl[1] = 0; sl[2] = 0; }
			BArray(arr)
		}
	}
	/// Sets the backing array to a new string.
	///
	/// Panics if string doesn't fit in the backing array.
	pub fn set<T: AsRef<OsStr>>(&mut self, sty: T) {
		self.set_from(sty.as_ref().encode_wide());
	}
	/// Sets the backing array to a new string.
	///
	/// Panics if string doesn't fit in the backing array.
	pub fn set_from<T: IntoIterator<Item = OLECHAR>>(&mut self, ty: T) {
		let mut sl = self.0.as_mut_slice();
		let mut n = 0;
		for ch in ty.into_iter() {
			sl[n + 2] = ch;
			n += 1;
		}
		sl[n + 2] = 0;
		unsafe { *(sl.as_mut_ptr() as *mut u32) = (n << 1) as u32; }
	}
	fn shrink(&self) -> &[OLECHAR] {
		unsafe {
			let sp = self.0.as_slice().as_ptr();
			let len = (*(sp as *const u32) >> 1) as usize;
			slice::from_raw_parts(sp, len.wrapping_add(3))
		}
	}
}
impl<A: Array<OLECHAR>> Default for BArray<A> {
	fn default() -> BArray<A> {
		BArray::new()
	}
}

//----------------------------------------------------------------
// From, FromIterator

impl<A: Array<OLECHAR>, T: AsRef<OsStr>> From<T> for BArray<A> {
	fn from(s: T) -> BArray<A> {
		let mut barr = BArray::new();
		barr.set(s);
		barr
	}
}
impl<A: Array<OLECHAR>> FromIterator<OLECHAR> for BArray<A> {
	fn from_iter<T: IntoIterator<Item = OLECHAR>>(ty: T) -> BArray<A> {
		let mut barr = BArray::new();
		barr.set_from(ty);
		barr
	}
}

//----------------------------------------------------------------
// AsRef, Deref, AsRawBStr

impl<A: Array<OLECHAR>> AsRef<BStr> for BArray<A> {
	fn as_ref(&self) -> &BStr {
		unsafe { BStr::new(self.shrink()) }
	}
}
impl<A: Array<OLECHAR>> ops::Deref for BArray<A> {
	type Target = BStr;
	fn deref(&self) -> &BStr {
		self.as_ref()
	}
}
impl<A: Array<OLECHAR>> AsRawBStr for BArray<A> {
	fn as_raw(&self) -> BSTR {
		unsafe { self.0.as_slice().as_ptr().offset(2) as BSTR }
	}
}

//----------------------------------------------------------------
// Debug

impl<A: Array<OLECHAR>> fmt::Debug for BArray<A> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(&**self).fmt(f)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use ::std::ffi::{OsStr};
	use super::super::array_traits::Array;
	use super::super::bstr::BStr;
	use super::*;
	use ::winapi::OLECHAR;
	fn equal<A: Array<OLECHAR>, T: AsRef<OsStr>>(sty: T) {
		let barr = BArray::<A>::from(sty.as_ref());
		let bstr: &BStr = barr.as_ref();
		assert_eq!(bstr, sty.as_ref());
	}
	#[test]
	fn units() {
		equal::<[_; 32], _>("hello, world :)");
		equal::<[_; 21], _>("filled to capacity");
	}
	#[test]
	#[should_panic(expected = "assertion failed: index <")]
	fn overflow() {
		equal::<[_; 10], _>("overflow");
	}
}
