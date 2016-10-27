use ::com_sys::{ComInterface};

/// Com pointers wrap the raw com interface pointers and give them a safe rustic interface.
///
/// By convention com pointers should be named after their interface name with `Ptr` added at the end.
pub trait ComPtr: AsComPtr {
	/// Constructs a new instance from void pointer.
	///
	/// # Safety
	///
	/// Must not be null, reinterprets the void pointer as a raw interface pointer.
	unsafe fn new(iface: ::LPVOID) -> Self;
	/// Constructs a new instance from raw interface pointer.
	///
	/// # Safety
	///
	/// Must not be null, should be a valid raw interface pointer.
	unsafe fn from_ptr(iface: *mut Self::Interface) -> Self;
	/// Cast the com pointer to a super class.
	#[inline(always)]
	fn cast<P: ComPtr>(&self) -> &P where Self: ComCast<P> {
		<Self as ComCast<P>>::cast(self)
	}
}

/// Access the com interface pointer.
pub trait AsComPtr: Sized {
	/// The com interface type.
	type Interface: ComInterface;
	/// Returns the interface pointer.
	///
	/// Use [`com_call!(VirtualFunction(this, args...)`](macro.com_call!.html) to call virtual functions.
	fn as_ptr(&self) -> *mut Self::Interface;
}
impl<'a, P: 'a + AsComPtr> AsComPtr for &'a P {
	type Interface = P::Interface;
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self::Interface {
		(**self).as_ptr()
	}
}
impl<'a, P: 'a + AsComPtr> AsComPtr for &'a mut P {
	type Interface = P::Interface;
	#[inline(always)]
	fn as_ptr(&self) -> *mut Self::Interface {
		(**self).as_ptr()
	}
}

/// Casts an instance to a related type.
pub trait ComCast<T: ComPtr>: AsComPtr {
	/// Perform the cast.
	fn cast(&self) -> &T;
}
impl<'a, S: 'a + ComCast<T>, T: ComPtr> ComCast<T> for &'a S {
	#[inline(always)]
	fn cast(&self) -> &T {
		(**self).cast()
	}
}
impl<'a, S: 'a + ComCast<T>, T: ComPtr> ComCast<T> for &'a mut S {
	#[inline(always)]
	fn cast(&self) -> &T {
		(**self).cast()
	}
}
