/*!
Safe array.
*/

use ::std::{fmt, mem};

mod safe_arr;
pub use self::safe_arr::*;

mod safe_array;
pub use self::safe_array::*;

mod typed_arr;
pub use self::typed_arr::*;

mod typed_array;
pub use self::typed_array::*;

mod slice;
pub use self::slice::*;

use ::winapi::{SAFEARRAY};
fn vartype(raw: *const SAFEARRAY) -> ElemTag {
	unsafe {
		let mut vt = mem::uninitialized();
		let hr = sys::SafeArrayGetVartype(raw, &mut vt);
		debug_assert_eq!(hr, 0);
		ElemTag(vt)
	}
}
fn assert_vartype(raw: *const SAFEARRAY, tag: ElemTag) {
	assert_eq!(vartype(raw), tag);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ElemTag(u16);
impl_inner_newtype!(ElemTag: u16);

pub trait ElemType: Sized {
	fn elem_tag() -> ElemTag;

	type Raw: Sized;
	type Ref: fmt::Debug + ?Sized;

	fn elem_ref(raw: &Self::Raw) -> &Self::Ref;
}

impl ElemType for ::bstr::BString {
	fn elem_tag() -> ElemTag {
		ElemTag(::winapi::VT_BSTR.0 as u16)
	}
	type Raw = ::winapi::BSTR;
	type Ref = ::bstr::BStr;

	fn elem_ref(raw: &Self::Raw) -> &Self::Ref {
		unsafe { ::bstr::BStr::from_raw(*raw) }
	}
}

pub mod sys;
