/*!
Contribute me back plz.
*/

use ::winapi::{SAFEARRAY, SAFEARRAYBOUND, LONG, ULONG, VARTYPE, UINT, HRESULT, PVOID, c_void, GUID, REFGUID};

extern "system" {
	pub fn SafeArrayAccessData(psa: *mut SAFEARRAY, ppvData: *mut *mut c_void) -> HRESULT;
	pub fn SafeArrayAllocData(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayAllocDescriptor(cDims: UINT, ppsaOut: *mut *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayAllocDescriptorEx(vt: VARTYPE, cDims: UINT, ppsaOut: *mut *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayCopy(psa: *mut SAFEARRAY, ppsaOut: *mut *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayCopyData(psaSource: *mut SAFEARRAY, psaTarget: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayCreate(vt: VARTYPE, cDims: UINT, rgsabound: *mut SAFEARRAYBOUND) -> *mut SAFEARRAY;
	pub fn SafeArrayCreateEx(vt: VARTYPE, cDims: UINT, rgsabound: *mut SAFEARRAYBOUND, pvExtra: PVOID) -> *mut SAFEARRAY;
	pub fn SafeArrayCreateVector(vt: VARTYPE, lLbound: LONG, cElements: ULONG) -> *mut SAFEARRAY;
	pub fn SafeArrayCreateVectorEx(vt: VARTYPE, lLbound: LONG, cElements: ULONG, pvExtra: PVOID) -> *mut SAFEARRAY;
	pub fn SafeArrayDestroy(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayDestroyData(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayDestroyDescriptor(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayGetDim(psa: *const SAFEARRAY) -> HRESULT;
	pub fn SafeArrayGetElement(psa: *const SAFEARRAY, rgIndices: *mut LONG, pv: *mut c_void) -> HRESULT;
	pub fn SafeArrayGetElemsize(psa: *const SAFEARRAY) -> UINT;
	pub fn SafeArrayGetIID(psa: *const SAFEARRAY, pguid: *mut GUID) -> HRESULT;
	pub fn SafeArrayGetLBound(psa: *const SAFEARRAY, nDim: UINT, plLbound: *mut LONG) -> HRESULT;
	// pub fn SafeArrayGetRecordInfo(psa: *mut SAFEARRAY, prinfo: *mut *mut IRecordInfo) -> HRESULT;
	pub fn SafeArrayGetUBound(psa: *const SAFEARRAY, nDim: UINT, plUbound: *mut LONG) -> HRESULT;
	pub fn SafeArrayGetVartype(psa: *const SAFEARRAY, pvt: *mut VARTYPE) -> HRESULT;
	pub fn SafeArrayLock(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayPtrOfIndex(psa: *mut SAFEARRAY, rgIndices: *mut LONG, ppvData: *mut *mut c_void) -> HRESULT;
	pub fn SafeArrayPutElement(psa: *mut SAFEARRAY, rgIndices: *mut LONG, pv: *mut c_void) -> HRESULT;
	pub fn SafeArrayRedim(psa: *mut SAFEARRAY, psaboundNew: *mut SAFEARRAYBOUND) -> HRESULT;
	pub fn SafeArraySetIID(psa: *mut SAFEARRAY, guid: REFGUID) -> HRESULT;
	// pub fn SafeArraySetRecordInfo(psa: *mut SAFEARRAY, prinfo: *mut IRecordInfo) -> HRESULT;
	pub fn SafeArrayUnaccessData(psa: *mut SAFEARRAY) -> HRESULT;
	pub fn SafeArrayUnlock(psa: *mut SAFEARRAY) -> HRESULT;
}
