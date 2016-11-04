/*!
COM initialization.
*/

use ::std::{ptr};

use ole32::{CoInitialize, CoInitializeEx, CoInitializeSecurity, CoUninitialize, CoCreateInstance};
use winapi::{RPC_C_AUTHN_LEVEL_DEFAULT, RPC_C_IMP_LEVEL_IMPERSONATE, EOAC_NONE, CLSCTX_INPROC_SERVER};
use winapi::{RPC_C_AUTHN_WINNT, RPC_C_AUTHZ_NONE, RPC_C_AUTHN_LEVEL_CALL};
use super::sys::{CoSetProxyBlanket};

use ::com_types::hr::{HResult};
use ::com_types::com::{ComResult, ComPtr, AsComPtr, ComCast};
use ::com_sys::{ComClassInterface, ComInterface};
use ::com_types::unknown::{IUnknownPtr};

bitflags! {
	pub flags CoInit: u32 {
		const COINIT_APARTMENTTHREADED = 0x2,
		const COINIT_MULTITHREADED = 0x0,
		const COINIT_DISABLE_OLE1DDE = 0x4,
		const COINIT_SPEED_OVER_MEMORY = 0x8,
	}
}

pub struct Com;
impl Com {
	/// Initializes the COM library on the current thread.
	///
	/// See msdn [CoInitialize](https://msdn.microsoft.com/en-us/library/windows/desktop/ms678543.aspx).
	pub fn initialize() -> ComResult<Com> {
		unsafe {
			let hr = CoInitialize(ptr::null_mut());
			HResult::result(hr, Com)
		}
	}

	/// Initializes the COM library for use by the calling thread.
	///
	/// See msdn [CoInitializeEx](https://msdn.microsoft.com/en-us/library/windows/desktop/ms695279.aspx).
	pub fn initialize_ex(flags: CoInit) -> ComResult<Com> {
		unsafe {
			let hr = CoInitializeEx(ptr::null_mut(), flags.bits());
			HResult::result(hr, Com)
		}
	}

	/// Registers security and sets the default security values for the process.
	///
	/// See msdn [CoInitializeSecurity](https://msdn.microsoft.com/en-us/library/windows/desktop/ms693736.aspx).
	pub fn initialize_security(self) -> ComResult<Com> {
		unsafe {
			let hr = CoInitializeSecurity(
				ptr::null_mut(),             // Security descriptor
				-1,                          // COM negotiates authentication service
				ptr::null_mut(),             // Authentication services
				ptr::null_mut(),             // Reserved
				RPC_C_AUTHN_LEVEL_DEFAULT,   // Default authentication level for proxies
				RPC_C_IMP_LEVEL_IMPERSONATE, // Default Impersonation level for proxies
				ptr::null_mut(),             // Authentication info
				EOAC_NONE,                   // Additional capabilities of the client or server
				ptr::null_mut(),             // Reserved
			);
			HResult::result(hr, self)
		}
	}

	/// Creates a single uninitialized object of the class associated with a specified CLSID.
	///
	/// See msdn [CoCreateInstance](https://msdn.microsoft.com/en-us/library/windows/desktop/ms686615.aspx).
	pub fn create_instance<P: ComPtr, C: ComClassInterface<P::Interface>>(&self) -> ComResult<P> {
		unsafe {
			let mut ppv = ptr::null_mut();
			let hr = CoCreateInstance(C::clsid(), ptr::null_mut(), CLSCTX_INPROC_SERVER, <P::Interface as ComInterface>::iid(), &mut ppv);
			HResult::result_of(hr, || P::new(ppv))
		}
	}

	/// Sets the authentication information that will be used to make calls on the specified proxy.
	///
	/// See msdn [CoSetProxyBlanket](https://msdn.microsoft.com/en-us/library/windows/desktop/ms692692.aspx).
	pub fn set_proxy_blanket<P: ComCast<IUnknownPtr>>(&self, proxy: P) -> ComResult<()> {
		unsafe {
			let hr = CoSetProxyBlanket(
				proxy.cast().as_ptr(),
				RPC_C_AUTHN_WINNT,
				RPC_C_AUTHZ_NONE,
				ptr::null_mut(),
				RPC_C_AUTHN_LEVEL_CALL,
				RPC_C_IMP_LEVEL_IMPERSONATE,
				ptr::null_mut(),
				EOAC_NONE
			);
			HResult::result(hr, ())
		}
	}
}
impl Drop for Com {
	fn drop(&mut self) {
		unsafe { CoUninitialize(); }
	}
}
