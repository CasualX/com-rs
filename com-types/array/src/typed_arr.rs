
use ::std::{slice, ops, mem};
use ::std::marker::PhantomData;

use ::winapi::{SAFEARRAY};

use ::sys::SafeArrayGetVartype;

use super::safe_arr::SafeArr;
use super::{ElemType, ElemTag};

#[repr(C)]
pub struct TypedArr<T: ElemType>(PhantomData<T>, SafeArr);
impl<T: ElemType> TypedArr<T> {
	pub unsafe fn from_raw<'ub>(raw: *const SAFEARRAY) -> &'ub TypedArr<T> {
		let mut var_type = mem::uninitialized();
		if SafeArrayGetVartype(raw, &mut var_type) < 0 || T::elem_tag() != ElemTag(var_type) {
			panic!("assertion failed: incorrect array element type, expected {:?} got {:?}", T::elem_tag(), ElemTag(var_type));
		}
		mem::transmute(SafeArr::from_raw(raw))
	}
	pub fn as_slice(&self) -> &[T::Raw] {
		let len = {
			let bounds = self.bounds();
			assert_eq!(bounds.len(), 1);
			bounds[0].cElements as usize
		};
		unsafe {
			let data = self.data() as *const T::Raw;
			slice::from_raw_parts(data, len)
		}
	}
	pub fn as_mut_slice(&mut self) -> &mut [T::Raw] {
		let len = {
			let bounds = self.bounds();
			assert_eq!(bounds.len(), 1);
			bounds[0].cElements as usize
		};
		unsafe {
			let data = self.data_mut() as *mut T::Raw;
			slice::from_raw_parts_mut(data, len)
		}
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
