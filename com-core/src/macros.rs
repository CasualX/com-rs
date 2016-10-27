/*!
Automate COM plumbing.

These macros automate the required plumbing to make everything work nicely.

Some work could be done to improve their ergonomics: allow trailing commas, attributes without keywords, ...

COM Rust macros
---------------

These help implementing the idiomatic Rust abstractions for the COM FFI.

* [`com_ptr!`](../macro.com_ptr!.html)

  Generates the scaffolding for implementing an idiomatic Rust interface for an interface.

  ```
  # #[macro_use] extern crate com_core; fn main() {
  use ::com_core::unknown::{IUnknown, IUnknownVtbl};

  com_interface! {
        interface IInterface(IInterfaceVtbl): IUnknown(IUnknownVtbl);
        {0xAAAAAAAA-0xBBBB-0xCCCC-0xDDDD-0xEEEEEEEEEEEE}
        pub Foo: unsafe extern "stdcall" fn(
            This: *mut IInterface,
        ),
  }

  use ::com_core::unknown::{IUnknownPtr};

  com_ptr! {
        #[derive(Clone, Debug)]
        pub struct IInterfacePtr(IInterface): IUnknownPtr;
  }
  # }
  ```

* [`com_call!`](../macro.com_call!.html)`(Foo(self))`

  Calls a virtual function on the COM interface.

  The first argument should evaulate to an impl `AsComPtr`.

*/

/// Defines the idiomatic Rust wrapper for a COM interface.
#[macro_export]
macro_rules! com_ptr {
	(
		$(#[$attr:meta])*
		pub struct $iface_ptr:ident($iface:ty);
	) => {
		$(#[$attr])*
		pub struct $iface_ptr(*mut $iface);
		impl $crate::AsComPtr for $iface_ptr {
			type Interface = $iface;
			#[inline(always)] fn as_ptr(&self) -> *mut Self::Interface { self.0 }
		}
		impl $crate::ComPtr for $iface_ptr {
			#[inline(always)]
			unsafe fn new(iface: $crate::LPVOID) -> Self {
				Self::from_ptr(iface as *mut Self::Interface)
			}
			#[inline(always)]
			unsafe fn from_ptr(iface: *mut $iface) -> Self {
				debug_assert!(!iface.is_null());
				$iface_ptr(iface)
			}
		}
	};
	(
		$(#[$attr:meta])*
		pub struct $iface_ptr:ident($iface:ty): $ibase_ptr:ty;
	) => {
		$(#[$attr])*
		pub struct $iface_ptr($ibase_ptr);
		impl $crate::AsComPtr for $iface_ptr {
			type Interface = $iface;
			#[inline(always)]
			fn as_ptr(&self) -> *mut Self::Interface { self.0.as_ptr() as *mut Self::Interface }
		}
		impl $crate::ComPtr for $iface_ptr {
			#[inline(always)]
			unsafe fn new(iface: $crate::LPVOID) -> Self {
				debug_assert!(!iface.is_null());
				$iface_ptr(<$ibase_ptr as $crate::ComPtr>::new(iface))
			}
			#[inline(always)]
			unsafe fn from_ptr(iface: *mut $iface) -> Self {
				Self::new(iface as $crate::LPVOID)
			}
		}
		impl $crate::ComCast<$ibase_ptr> for $iface_ptr {
			#[inline(always)] fn cast(&self) -> &$ibase_ptr { &self.0 }
		}
		impl ::std::ops::Deref for $iface_ptr {
			type Target = $ibase_ptr;
			fn deref(&self) -> &$ibase_ptr { &self.0 }
		}
	}
}

/// Calls a virtual function from a COM interface.
#[macro_export]
macro_rules! com_call {
	($vfn:ident($this:expr $(,$args:expr)*)) => {{
		let this_ptr = $crate::AsComPtr::as_ptr($this);
		let vtbl = $crate::ComInterface::vtbl(&*this_ptr);
		(vtbl.$vfn)(this_ptr $(,$args)*)
	}};
}
