
use ::std::{ops, borrow, mem};
use ::std::marker::PhantomData;

use ::winapi::{SAFEARRAY};

use super::sys::{SafeArrayDestroy};

use super::{ElemType, TypedArr};

pub struct TypedArray<T: ElemType>(*mut SAFEARRAY, PhantomData<T>);
impl<T: ElemType> TypedArray<T> {
	pub unsafe fn from_raw(raw: *mut SAFEARRAY) -> TypedArray<T> {
		super::assert_vartype(raw, T::elem_tag());
		TypedArray(raw, PhantomData)
	}
	pub fn into_raw(self) -> *mut SAFEARRAY {
		let raw = self.0;
		mem::forget(self);
		raw
	}
}
impl<T: ElemType> Drop for TypedArray<T> {
	fn drop(&mut self) {
		unsafe {
			let _ = SafeArrayDestroy(self.0);
		}
	}
}

//----------------------------------------------------------------

impl<T: ElemType> AsRef<TypedArr<T>> for TypedArray<T> {
	fn as_ref(&self) -> &TypedArr<T> {
		unsafe { TypedArr::from_raw(self.0) }
	}
}
impl<T: ElemType> ops::Deref for TypedArray<T> {
	type Target = TypedArr<T>;
	fn deref(&self) -> &TypedArr<T> {
		unsafe { TypedArr::from_raw(self.0) }
	}
}
impl<T: ElemType> borrow::Borrow<TypedArr<T>> for TypedArray<T> {
	fn borrow(&self) -> &TypedArr<T> {
		unsafe { TypedArr::from_raw(self.0) }
	}
}
impl<T: ElemType> borrow::BorrowMut<TypedArr<T>> for TypedArray<T> {
	fn borrow_mut(&mut self) -> &mut TypedArr<T> {
		unsafe { TypedArr::from_mut(self.0) }
	}
}

impl<T: ElemType> ::AsInner<*mut SAFEARRAY> for TypedArray<T> {
	fn as_inner(&self) -> &*mut SAFEARRAY {
		&self.0
	}
}
impl<T: ElemType> ::AsInnerMut<*mut SAFEARRAY> for TypedArray<T> {
	unsafe fn as_inner_mut(&mut self) -> &mut *mut SAFEARRAY {
		&mut self.0
	}
}
