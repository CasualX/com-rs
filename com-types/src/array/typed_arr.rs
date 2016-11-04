
use ::std::{slice, ops, mem};
use ::std::marker::PhantomData;

use ::winapi::{SAFEARRAY};

use super::{ElemType, SafeArr, ArrSlice};

#[repr(C)]
pub struct TypedArr<T: ElemType>(PhantomData<T>, SafeArr);
impl<T: ElemType> TypedArr<T> {
	pub unsafe fn from_raw<'ub>(raw: *const SAFEARRAY) -> &'ub TypedArr<T> {
		mem::transmute(SafeArr::from_raw(raw))
	}
	pub unsafe fn from_mut<'ub>(raw: *mut SAFEARRAY) -> &'ub mut TypedArr<T> {
		mem::transmute(SafeArr::from_mut(raw))
	}
	pub fn try_as_slice(&self) -> Option<&ArrSlice<T>> {
		let len = {
			let bounds = self.bounds();
			if bounds.len() != 1 {
				return None;
			}
			bounds[0].cElements as usize
		};
		unsafe {
			let data = self.data() as *const T::Raw;
			Some(<_ as ::FromInner<_>>::from_inner(slice::from_raw_parts(data, len)))
		}
	}
	pub fn as_slice(&self) -> &ArrSlice<T> {
		self.try_as_slice().expect("cannot slice multi-dimensional array")
	}
}

//----------------------------------------------------------------

impl<T: ElemType> AsRef<SafeArr> for TypedArr<T> {
	fn as_ref(&self) -> &SafeArr {
		&self.1
	}
}
impl<T: ElemType> AsMut<SafeArr> for TypedArr<T> {
	fn as_mut(&mut self) -> &mut SafeArr {
		&mut self.1
	}
}
impl<T: ElemType> ops::Deref for TypedArr<T> {
	type Target = SafeArr;
	fn deref(&self) -> &SafeArr {
		&self.1
	}
}
impl<T: ElemType> ops::DerefMut for TypedArr<T> {
	fn deref_mut(&mut self) -> &mut SafeArr {
		&mut self.1
	}
}
