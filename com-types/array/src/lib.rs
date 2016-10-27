/*!
*/

extern crate winapi;
extern crate oleaut32;
#[macro_use]
extern crate bitflags;

extern crate bstr;

mod safe_arr;
pub use safe_arr::*;

mod safe_array;
pub use safe_array::*;

mod typed_arr;
pub use typed_arr::*;

mod typed_array;
pub use typed_array::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ElemTag(u16);

pub trait ElemType: Sized {
	fn elem_tag() -> ElemTag;

	type Raw: Sized;
}

impl ElemType for bstr::BString {
	fn elem_tag() -> ElemTag {
		ElemTag(winapi::VT_BSTR.0 as u16)
	}
	type Raw = winapi::BSTR;
}

pub mod sys;
