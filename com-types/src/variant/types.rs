/*!
Variant types.
*/

use ::winapi::{CY, DATE, SCODE, BSTR, DECIMAL, SAFEARRAY};

use super::{vt, Type, Variant, Tag};

//----------------------------------------------------------------

/// Empty variant.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Empty;

impl_type_unit!(VT_EMPTY, Empty);

/// Null variant.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Null;

impl_type_unit!(VT_NULL, Null);

//----------------------------------------------------------------

/// Signed word variant.
pub type I2 = i16;
/// Signed dword variant.
pub type I4 = i32;
/// Float variant.
pub type R4 = f32;
/// Double variant.
pub type R8 = f64;

impl_type_native!(VT_I2, I2);
impl_type_native!(VT_I4, I4);
impl_type_native!(VT_R4, R4);
impl_type_native!(VT_R8, R8);

impl_type_ref_native!(I2);
impl_type_ref_native!(I4);
impl_type_ref_native!(R4);
impl_type_ref_native!(R8);

//----------------------------------------------------------------

pub type Currency = ::currency::Currency;
pub type Date = ::date::Date;

impl_type_reprc!(VT_CY, CY, ::currency::Currency);
impl_type_ref_reprc!(Currency);

impl_type_reprc!(VT_DATE, DATE, ::date::Date);
impl_type_ref_reprc!(Date);

//----------------------------------------------------------------
// BString

pub type BString = ::bstr::BString;

use ::bstr::{BStr};
unsafe impl<'a> Type<'a> for BString {
	fn variant_tag() -> Tag {
		vt::VT_BSTR
	}
	type Raw = BSTR;
	type Borrowed = BStr;
	type Owned = BString;
	unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s BStr {
		let raw = Self::variant_raw_borrow(var);
		BStr::from_raw(*raw)
	}
	unsafe fn variant_into(var: Variant<'a>) -> BString {
		let raw = Self::variant_raw_into(var);
		BString::from_raw(raw)
	}
	unsafe fn variant_from(self) -> Variant<'a> {
		Self::variant_raw_from(self.into_raw())
	}
}
unsafe impl<'a> Type<'a> for &'a BString {
	fn variant_tag() -> Tag {
		vt::VT_BYREF_BSTR
	}
	type Raw = *const BSTR;
	type Borrowed = BStr;
	type Owned = &'a BStr;
	unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s BStr {
		let raw = Self::variant_raw_borrow(var);
		BStr::from_raw(**raw)
	}
	unsafe fn variant_into(var: Variant<'a>) -> &'a BStr {
		let raw = Self::variant_raw_into(var);
		BStr::from_raw(*raw)
	}
	unsafe fn variant_from(self) -> Variant<'a> {
		use ::AsInner;
		Self::variant_raw_from(self.as_inner())
	}
}

//----------------------------------------------------------------

pub type Error = ::hr::SCode;
pub type Bool = ::bool::Bool;

unsafe impl<'a> Type<'a> for Error {
	fn variant_tag() -> Tag {
		vt::VT_ERROR
	}
	type Raw = SCODE;
	type Borrowed = Error;
	type Owned = Error;
	unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s Error {
		&*(Self::variant_raw_borrow(var) as *const _ as *const Error)
	}
	unsafe fn variant_into(var: Variant<'a>) -> Error {
		Self::variant_raw_into(var).into()
	}
	unsafe fn variant_from(self) -> Variant<'a> {
		Self::variant_raw_from(self.into())
	}
}
impl_type_ref_reprc!(Error);

impl_type_reprc!(VT_BOOL, u16, ::bool::Bool);
impl_type_ref_reprc!(Bool);

pub type Decimal = ::decimal::Decimal;
impl_type_reprc!(VT_DECIMAL, DECIMAL, ::decimal::Decimal);
impl_type_ref_reprc!(Decimal);

//----------------------------------------------------------------

/// Signed byte variant.
pub type I1 = i8;
/// Unsigned byte variant.
pub type UI1 = u8;
/// Unsigned word variant.
pub type UI2 = u16;
/// Unsigned dword variant.
pub type UI4 = u32;

impl_type_native!(VT_I1, I1);
impl_type_native!(VT_UI1, UI1);
impl_type_native!(VT_UI2, UI2);
impl_type_native!(VT_UI4, UI4);

impl_type_ref_native!(I1);
impl_type_ref_native!(UI1);
impl_type_ref_native!(UI2);
impl_type_ref_native!(UI4);

//----------------------------------------------------------------

/// Unspecified integer.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Int(i32);
impl_inner_newtype!(Int: i32);

impl_type_reprc!(VT_INT, i32, Int);
impl_type_ref_reprc!(Int);

/// Unspecified unsigned integer.
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct UInt(u32);
impl_inner_newtype!(UInt: u32);

impl_type_reprc!(VT_UINT, u32, UInt);
impl_type_ref_reprc!(UInt);

//----------------------------------------------------------------

use ::array::{ElemType, TypedArr, TypedArray};

unsafe impl<'a, T: 'a + ElemType> Type<'a> for TypedArray<T> {
	fn variant_tag() -> Tag {
		use ::IntoInner;
		Tag(T::elem_tag().into_inner() | vt::VT_ARRAY)
	}
	type Raw = *mut SAFEARRAY;
	type Borrowed = TypedArr<T>;
	type Owned = TypedArray<T>;
	unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s TypedArr<T> {
		let raw = Self::variant_raw_borrow(var);
		TypedArr::from_raw(*raw)
	}
	unsafe fn variant_into(var: Variant<'a>) -> TypedArray<T> {
		let raw = Self::variant_raw_into(var);
		TypedArray::from_raw(raw)
	}
	unsafe fn variant_from(self) -> Variant<'a> {
		Self::variant_raw_from(self.into_raw())
	}
}
unsafe impl<'a, T: 'a + ElemType> Type<'a> for &'a TypedArray<T> {
	fn variant_tag() -> Tag {
		use ::IntoInner;
		Tag(T::elem_tag().into_inner() | vt::VT_BYREF | vt::VT_ARRAY)
	}
	type Raw = *const *mut SAFEARRAY;
	type Borrowed = TypedArr<T>;
	type Owned = &'a TypedArr<T>;
	unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s TypedArr<T> {
		let raw = Self::variant_raw_borrow(var);
		TypedArr::from_raw(**raw)
	}
	unsafe fn variant_into(var: Variant<'a>) -> &'a TypedArr<T> {
		let raw = Self::variant_raw_into(var);
		TypedArr::from_raw(*raw)
	}
	unsafe fn variant_from(self) -> Variant<'a> {
		use ::AsInner;
		Self::variant_raw_from(self.as_inner())
	}
}
