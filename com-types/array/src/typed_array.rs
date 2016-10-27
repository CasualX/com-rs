
use ::std::marker::PhantomData;

use ::winapi::{SAFEARRAY};

use ::sys::{SafeArrayDestroy};

use super::ElemType;
use super::typed_arr::TypedArr;

pub struct TypedArray<T: ElemType>(*mut SAFEARRAY, PhantomData<T>);
impl<T: ElemType> TypedArray<T> {
	
}
impl<T: ElemType> Drop for TypedArray<T> {
	fn drop(&mut self) {
		unsafe {
			let _ = SafeArrayDestroy(self.0);
		}
	}
}
