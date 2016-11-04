/*!
This should be part of the std lib...
*/

use ::std::slice::{from_raw_parts, from_raw_parts_mut};

pub trait Array<T> {
	fn as_slice(&self) -> &[T];
	fn as_mut_slice(&mut self) -> &mut [T];
	fn capacity() -> usize;
}
impl<A: FixedSizeArray> Array<A::Item> for A {
	fn as_slice(&self) -> &[A::Item] { unsafe { from_raw_parts(self.as_ptr(), A::capacity()) } }
	fn as_mut_slice(&mut self) -> &mut [A::Item] { unsafe { from_raw_parts_mut(self.as_mut_ptr(), A::capacity()) } }
	fn capacity() -> usize { A::capacity() }
}

pub unsafe trait FixedSizeArray {
	type Item;
	fn as_ptr(&self) -> *const Self::Item;
	fn as_mut_ptr(&mut self) -> *mut Self::Item;
	fn capacity() -> usize;
}

macro_rules! impl_fixed_size_array {
	($len:expr) => {
		unsafe impl<T> FixedSizeArray for [T; $len] {
			type Item = T;
			#[inline(always)] fn as_ptr(&self) -> *const T { self as *const _ as *const T }
			#[inline(always)] fn as_mut_ptr(&mut self) -> *mut T { self as *mut _ as *mut T }
			#[inline(always)] fn capacity() -> usize { $len }
		}
	}
}

macro_rules! impl_fixed_size_arrays {
	() => ();
	($len:expr, $($tail:expr,)*) => {
		impl_fixed_size_arrays! { $($tail,)* }
		impl_fixed_size_array!($len);
	}
}

impl_fixed_size_arrays! {
	0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
	16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31,
	32, 40, 48, 56, 64, 72, 96, 128, 160, 192, 224,
	256, 384, 512, 768, 1024, 2048, 4096, 8192, 16384, 32768,
}

#[cfg(test)]
mod tests {
	use super::*;
	fn takes_array_i32<A: Array<i32>>(arr: &A) { arr.as_slice(); }
	fn takes_mut_array_i32<A: Array<i32>>(arr: &mut A) { arr.as_mut_slice(); }
	#[test]
	fn units() {
		let mut a: [i32; 768] = unsafe { ::std::mem::uninitialized() };
		assert_eq!(a.as_slice().len(), 768);
		takes_array_i32(&a);
		takes_mut_array_i32(&mut a);
	}
}
