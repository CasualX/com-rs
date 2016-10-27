
use ::std::{fmt, slice, mem};
use ::std::os::raw::c_void;

use ::winapi::{USHORT, ULONG, PVOID, SAFEARRAY, SAFEARRAYBOUND};

use ::sys::{SafeArrayGetVartype};

use super::{ElemType, ElemTag};
use super::typed_arr::TypedArr;

pub trait AsRawArray {
	fn as_raw(&self) -> *const SAFEARRAY;
	fn as_raw_mut(&mut self) -> *mut SAFEARRAY;
}

#[repr(C)]
pub struct SafeArr {
	dims: USHORT,
	features: USHORT,
	elements: ULONG,
	locks: ULONG,
	data: PVOID,
	bounds: [SAFEARRAYBOUND],
}
impl SafeArr {
	pub unsafe fn from_raw<'ub>(raw: *const SAFEARRAY) -> &'ub SafeArr {
		let dims = (*raw).cDims as usize;
		// Construct the DST...
		mem::transmute(slice::from_raw_parts(raw as *const SAFEARRAYBOUND, dims))
	}
	pub fn elem_tag(&self) -> ElemTag {
		unsafe {
			let mut vt = mem::uninitialized();
			let hr = SafeArrayGetVartype(self.as_raw(), &mut vt);
			debug_assert!(hr == 0);
			ElemTag(vt)
		}
	}
	pub fn data(&self) -> *const c_void {
		self.data
	}
	pub fn data_mut(&mut self) -> *mut c_void {
		self.data
	}
	pub fn try_as_typed<T: ElemType>(&self) -> Option<&TypedArr<T>> {
		if self.elem_tag() == T::elem_tag() {
			Some(unsafe { mem::transmute(self) })
		}
		else {
			None
		}
	}
	pub fn as_typed<T: ElemType>(&self) -> &TypedArr<T> {
		assert_eq!(self.elem_tag(), T::elem_tag());
		unsafe { mem::transmute(self) }
	}
	pub fn dims(&self) -> usize {
		self.dims as usize
	}
	pub fn features(&self) -> USHORT {
		self.features
	}
	pub fn bounds(&self) -> &[SAFEARRAYBOUND] {
		&self.bounds
	}
}

//----------------------------------------------------------------

impl AsRawArray for SafeArr {
	fn as_raw(&self) -> *const SAFEARRAY {
		self as *const _ as *const SAFEARRAY
	}
	fn as_raw_mut(&mut self) -> *mut SAFEARRAY {
		self as *mut _ as *mut SAFEARRAY
	}
}

//----------------------------------------------------------------
// Debug

impl fmt::Debug for SafeArr {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_struct("SafeArr")
			.field("cDims", &self.dims)
			.field("fFeatures", &self.features)
			.field("cbElements", &self.elements)
			.field("cLocks", &self.locks)
			.field("pvData", &self.data)
			.field("bounds", &&self.bounds)
			.finish()
	}
}
