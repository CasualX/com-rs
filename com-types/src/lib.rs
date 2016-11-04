/*!
*/

#![cfg(windows)]

extern crate winapi;
extern crate oleaut32;

#[macro_use]
extern crate bitflags;

extern crate com_sys;

//----------------------------------------------------------------

#[doc(hidden)]
pub use winapi::{LPVOID, GUID};

#[doc(hidden)]
pub use com_sys::{ComVtbl, ComInterface, ComClass, ComClassInterface};

//----------------------------------------------------------------

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

//----------------------------------------------------------------

#[macro_use]
pub mod macros;

pub mod array;
pub mod bool;
pub mod bstr;
pub mod com;
pub mod currency;
pub mod date;
pub mod decimal;
pub mod hr;
pub mod timeout;
pub mod variant;
pub mod unknown;
