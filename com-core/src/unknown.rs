/*!
Implements `IUnknown`.
*/

use ::hr::{HResult};
use ::{ComResult, ComPtr};
use ::com_sys::unknown::IUnknown;
use ::com_sys::ComInterface;

com_ptr! {
	#[derive(Debug)]
	pub struct IUnknownPtr(IUnknown);
}
impl Clone for IUnknownPtr {
	fn clone(&self) -> IUnknownPtr {
		unsafe {
			com_call!(AddRef(self));
			IUnknownPtr(self.0)
		}
	}
}
impl Drop for IUnknownPtr {
	fn drop(&mut self) {
		unsafe {
			com_call!(Release(self));
		}
	}
}
impl IUnknownPtr {
	pub fn query_interface<T: ComPtr>(&self) -> ComResult<T> {
		unsafe {
			let mut ppv = ::std::ptr::null_mut();
			let hr = com_call!(QueryInterface(self, <T::Interface as ComInterface>::iid(), &mut ppv));
			HResult::result_of(hr, || T::new(ppv))
		}
	}
}
