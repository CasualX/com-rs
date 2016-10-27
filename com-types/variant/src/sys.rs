
use ::winapi::{HRESULT, VARIANTARG, USHORT, VARTYPE, LCID};

extern "system" {
	pub fn VariantChangeType(pvargDest: *mut VARIANTARG, pvargSrc: *const VARIANTARG, wFlags: USHORT, vt: VARTYPE) -> HRESULT;
	pub fn VariantChangeTypeEx(pvargDest: *mut VARIANTARG, pvargSrc: *const VARIANTARG, lcid: LCID, wFlags: USHORT, vt: VARTYPE) -> HRESULT;
	pub fn VariantClear(pvarg: *mut VARIANTARG) -> HRESULT;
	pub fn VariantCopy(pvargDest: *mut VARIANTARG, pvargSrc: *const VARIANTARG) -> HRESULT;
	pub fn VariantCopyInd(pvargDest: *mut VARIANTARG, pvargSrc: *const VARIANTARG) -> HRESULT;
	pub fn VariantInit(pvarg: *mut VARIANTARG);
}
