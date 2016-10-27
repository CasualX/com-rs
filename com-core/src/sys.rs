/*!
Some of the missing stuff from ole32's bindings.
*/

use ::winapi::{DWORD, HRESULT, OLECHAR, RPC_AUTH_IDENTITY_HANDLE};
use ::com_sys::unknown::{IUnknown};

extern "system" {
	pub fn CoSetProxyBlanket(pProxy: *mut IUnknown, dwAuthnSvc: DWORD, dwAuthzSvc: DWORD, pServerPrincName: *mut OLECHAR, dwAuthnLevel: DWORD, dwImpLevel: DWORD, pAuthInfo: RPC_AUTH_IDENTITY_HANDLE, dwCapabilities: DWORD) -> HRESULT;
}
