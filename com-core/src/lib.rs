/*!

Build idiomatic Rust bindings for COM interfaces.

# Examples

```
# extern crate winapi; extern crate hresult as hr; #[macro_use] extern crate com_core; fn main() {
use ::winapi::HRESULT;
use ::hr::HResult;

use ::com_core::ComResult;
use ::com_core::com::Com;

// Define the COM FFI interface struct and its vtbl struct.

use ::com_core::unknown::{IUnknown, IUnknownVtbl};
com_interface! {
	/// Doc comment.
	interface IInterface(IInterfaceVtbl) : IUnknown(IUnknownVtbl);
	{0x582cdb5a-0xe8b9-0x47bb-0xbff8-0x5a18da55bc99}
	pub Foo: unsafe extern "stdcall" fn(
		This: *mut IInterface,
	) -> HRESULT,
}

// Define and implement the Rust binding for this interface.

use ::com_core::unknown::IUnknownPtr;
com_ptr! {
	/// Doc comment.
	#[derive(Clone, Debug)]
	pub struct IInterfacePtr(IInterface) : IUnknownPtr;
}

impl IInterfacePtr {
	pub fn foo(&self) -> ComResult<()> {
		unsafe {
			let hr: HResult = com_call!(Foo(self)).into();
			hr.result(())
		}
	}
}

// An externally defined com class which implements `IInterface`.

com_class!(_extern Interface, {0xf08b1ee5-0xc953-0x4faf-0x813c-0x9bd256d6fe48}, IInterface);

// Initialize COM and create an com pointer instance for this class.

let com = Com::initialize().unwrap();
let interface_ptr = com.create_instance::<IInterfacePtr, Interface>();

// The above will fail because there's really no such class, but you get the point.
assert!(interface_ptr.is_err());
# }
```
*/

#![cfg(windows)]

extern crate winapi;
extern crate ole32;
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate com_sys;

mod sys;

pub extern crate hresult as hr;
pub type ComResult<T> = Result<T, hr::HResult>;

pub use winapi::{LPVOID, GUID};

#[macro_use]
pub mod macros;

mod traits;
pub use traits::*;

pub mod com;

pub mod unknown;

pub mod timeout;

#[doc(hidden)]
pub use com_sys::{ComVtbl, ComInterface, ComClass, ComClassInterface};
