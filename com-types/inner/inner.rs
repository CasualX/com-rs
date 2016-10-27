/*!
*/

pub trait AsInner<T: ?Sized> {
	fn as_inner(&self) -> &T;
}
pub trait AsInnerMut<T: ?Sized>: AsInner<T> {
	unsafe fn as_inner_mut(&mut self) -> &mut T;
}

pub trait IntoInner<T> {
	fn into_inner(self) -> T;
}
pub trait FromInner<T> {
	unsafe fn from_inner(inner: T) -> Self;
}

#[macro_export]
macro_rules! impl_inner_newtype {
	($ty:path: $inner:ty) => {
		impl $crate::AsInner<$inner> for $ty {
			fn as_inner(&self) -> &$inner { &self.0 }
		}
		impl $crate::AsInnerMut<$inner> for $ty {
			unsafe fn as_inner_mut(&mut self) -> &mut $inner { &mut self.0 }
		}
		impl $crate::IntoInner<$inner> for $ty {
			fn into_inner(self) -> $inner { self.0 }
		}
		impl $crate::FromInner<$inner> for $ty {
			unsafe fn from_inner(inner: $inner) -> $ty { $ty(inner) }
		}
	}
}
