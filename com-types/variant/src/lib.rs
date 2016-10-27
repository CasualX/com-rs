/*!
Idiomatic Rust bindings for COM `VARIANT`.
*/

#![cfg(windows)]

extern crate winapi;
extern crate oleaut32;

extern crate bool;
extern crate bstr;
extern crate currency;
extern crate date;
extern crate decimal;
extern crate hresult;
#[macro_use]
extern crate inner;
extern crate array;

use ::std::{fmt, ptr, mem};
use ::std::marker::PhantomData;

use winapi::{VARIANT};

use hresult::HResult;

#[macro_use]
mod macros;

pub mod types;

pub mod sys;
use sys::{VariantInit, VariantClear, VariantCopy, VariantCopyInd};

//----------------------------------------------------------------

pub mod vt;

/// The tag declares the type contained in a variant.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct Tag(u16);

//----------------------------------------------------------------

/// COM `VARIANT`.
pub struct Variant<'v>(VARIANT, PhantomData<&'v ()>);
impl<'v> Variant<'v> {
	/// Returns a new `Empty` variant.
	pub fn new() -> Variant<'v> {
		Variant(VARIANT { data0: 0, data1: 0, data2: 0 }, PhantomData)
	}
	/// Creates an initialized variant.
	///
	/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221402.aspx).
	pub fn init() -> Variant<'v> {
		unsafe {
			let mut var = Variant(mem::uninitialized(), PhantomData);
			VariantInit(&mut var.0);
			var
		}
	}
	/// Takes ownership of a raw `VARIANT`.
	pub unsafe fn from_raw<'ub>(raw: VARIANT) -> Variant<'ub> {
		Variant(raw, PhantomData)
	}
	/// Surrenders ownership of the `VARIANT` for FFI.
	///
	/// The caller is responsible for clearing the variant or memory will be leaked.
	pub fn into_raw(self) -> VARIANT {
		let raw = self.0;
		mem::forget(self);
		raw
	}
	/// Returns a raw pointer to the contained `VARIANT` for FFI.
	pub fn as_ptr(&self) -> *const VARIANT {
		&self.0
	}
	/// Returns an unsafe mutable pointer to the contained `VARIANT` for FFI.
	pub fn as_mut_ptr(&mut self) -> *mut VARIANT {
		&mut self.0
	}
	/// Tries to clear the variant and resets it to `Empty`.
	///
	/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221165.aspx).
	pub fn try_clear(&mut self) -> Result<(), HResult> {
		unsafe {
			let hr = VariantClear(&mut self.0);
			HResult::result(hr, ())
		}
	}
	/// Tries to create a copy of the variant.
	///
	/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221697.aspx).
	pub fn try_clone(&self) -> Result<Variant, HResult> {
		unsafe {
			let mut dest = Variant::init().into_raw();
			let hr = VariantCopy(&mut dest, &self.0);
			HResult::result_of(hr, || Variant::from_raw(dest))
		}
	}
	/// Tries to copy-assign from `source`.
	///
	/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221697.aspx).
	pub fn try_clone_from(&mut self, source: &Variant) -> Result<(), HResult> {
		unsafe {
			let hr = VariantCopy(&mut self.0, &source.0);
			HResult::result(hr, ())
		}
	}
	/// Tries to create a copy of the variant, dereferencing if necessary.
	///
	/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221184.aspx).
	pub fn try_clone_ind(&self) -> Result<Variant<'static>, HResult> {
		unsafe {
			let mut dest = Variant::init().into_raw();
			let hr = VariantCopyInd(&mut dest, &self.0);
			HResult::result_of(hr, || Variant::from_raw(dest))
		}
	}
	/// Tries to copy-assign from `source`, dereferencing if necessary.
	///
	/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221184.aspx).
	pub fn try_clone_ind_from(&mut self, source: &Variant) -> Result<(), HResult> {
		unsafe {
			let hr = VariantCopyInd(&mut self.0, &source.0);
			HResult::result(hr, ())
		}
	}
	/// Returns the tag of the variant.
	pub fn tag(&self) -> Tag {
		unsafe { mem::transmute_copy(&self.0.data0) }
	}
	/// Returns if the variant contains the specified `T`ype.
	pub fn is<T: Type<'v>>(&self) -> bool {
		self.tag() == T::variant_tag()
	}
	/// Returns a reference to the underlying data if this variant is tagged as such.
	pub fn try_borrow<T: Type<'v>>(&self) -> Option<&T::Borrowed> {
		if T::variant_tag() == self.tag() {
			Some(unsafe { T::variant_borrow(self) })
		}
		else {
			None
		}
	}
	/// Returns a reference as the specified type to the underlying data.
	///
	/// Panics if the variant's tag is not correct for the type.
	pub fn borrow<T: Type<'v>>(&self) -> &T::Borrowed {
		assert_eq!(T::variant_tag(), self.tag());
		unsafe { T::variant_borrow(self) }
	}
	/// Returns the underlying data if this variant is tagged as such.
	pub fn try_into<T: Type<'v>>(self) -> Result<T::Owned, Variant<'v>> {
		if T::variant_tag() == self.tag() {
			Ok(unsafe { T::variant_into(self) })
		}
		else {
			Err(self)
		}
	}
	/// Returns the variant as the specified type.
	///
	/// Panics if the variant's tag is not correct for the type.
	pub fn into<T: Type<'v>>(self) -> T::Owned {
		assert_eq!(T::variant_tag(), self.tag());
		unsafe { T::variant_into(self) }
	}
}
impl<'v> Drop for Variant<'v> {
	fn drop(&mut self) {
		self.try_clear().unwrap();
	}
}
impl<'v> Default for Variant<'v> {
	fn default() -> Variant<'v> {
		Variant::init()
	}
}
impl<'v, T: Type<'v>> From<T> for Variant<'v> {
	fn from(val: T) -> Variant<'v> {
		unsafe { T::variant_from(val) }
	}
}
impl<'v> fmt::Debug for Variant<'v> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self.tag() {
			vt::VT_EMPTY => write!(f, "Variant({:?})", self.borrow::<types::Empty>()),
			vt::VT_NULL => write!(f, "Variant({:?})", self.borrow::<types::Null>()),
			vt::VT_I2 => write!(f, "Variant(I2({:?}))", self.borrow::<types::I2>()),
			vt::VT_I4 => write!(f, "Variant(I4({:?}))", self.borrow::<types::I4>()),
			vt::VT_R4 => write!(f, "Variant(R4({:?}))", self.borrow::<types::R4>()),
			vt::VT_R8 => write!(f, "Variant(R8({:?}))", self.borrow::<types::R8>()),
			vt::VT_CY => write!(f, "Variant({:?})", self.borrow::<types::Currency>()),
			vt::VT_DATE => write!(f, "Variant({:?})", self.borrow::<types::Date>()),
			vt::VT_BSTR => write!(f, "Variant({:?})", &self.borrow::<types::BString>()),
			// vt::VT_DISPATCH => write!(f, "Variant(Dispatch({:?}))", ),
			vt::VT_ERROR => write!(f, "Variant(Error({:?}))", self.borrow::<types::Error>()),
			vt::VT_BOOL => write!(f, "Variant({:?})", self.borrow::<types::Bool>()),
			// vt::VT_UNKNOWN => write!(f, "Variant(Unknown({:?}))", ),
			vt::VT_DECIMAL => write!(f, "Variant({:?})", self.borrow::<types::Decimal>()),
			vt::VT_I1 => write!(f, "Variant(I1({:?}))", self.borrow::<types::I1>()),
			vt::VT_UI1 => write!(f, "Variant(UI1({:?}))", self.borrow::<types::UI1>()),
			vt::VT_UI2 => write!(f, "Variant(UI2({:?}))", self.borrow::<types::UI2>()),
			vt::VT_UI4 => write!(f, "Variant(UI4({:?}))", self.borrow::<types::UI4>()),
			vt::VT_INT => write!(f, "Variant(Int({:?}))", self.borrow::<types::Int>()),
			vt::VT_UINT => write!(f, "Variant(UInt({:?}))", self.borrow::<types::UInt>()),
			// vt::VT_RECORD => f.field("record", ),
			vt::VT_BYREF_I2 => write!(f, "Variant(&I2({:?}))", self.borrow::<&types::I2>()),
			vt::VT_BYREF_I4 => write!(f, "Variant(&I4({:?}))", self.borrow::<&types::I4>()),
			vt::VT_BYREF_R4 => write!(f, "Variant(&R4({:?}))", self.borrow::<&types::R4>()),
			vt::VT_BYREF_R8 => write!(f, "Variant(&R8({:?}))", self.borrow::<&types::R8>()),
			vt::VT_BYREF_CY => write!(f, "Variant(&{:?})", self.borrow::<&types::Currency>()),
			vt::VT_BYREF_DATE => write!(f, "Variant(&{:?})", self.borrow::<&types::Date>()),
			vt::VT_BYREF_BSTR => write!(f, "Variant(&{:?})", &self.borrow::<&types::BString>()),
			// vt::VT_BYREF_DISPATCH => f.field("&dispatch", ),
			vt::VT_BYREF_ERROR => write!(f, "Variant(&Error({:?}))", self.borrow::<&types::Error>()),
			vt::VT_BYREF_BOOL => write!(f, "Variant(&{:?})", self.borrow::<types::Bool>()),
			// vt::VT_BYREF_VARIANT => f.field("&variant", self.borrow<&types::Variant>()),
			// vt::VT_BYREF_UNKNOWN => f.field("&unknown", ),
			vt::VT_BYREF_DECIMAL => write!(f, "Variant(&{:?})", self.borrow::<&types::Decimal>()),
			vt::VT_BYREF_I1 => write!(f, "Variant(&I1({:?}))", self.borrow::<&types::I1>()),
			vt::VT_BYREF_UI1 => write!(f, "Variant(&UI1({:?}))", self.borrow::<&types::UI1>()),
			vt::VT_BYREF_UI2 => write!(f, "Variant(&UI2({:?}))", self.borrow::<&types::UI2>()),
			vt::VT_BYREF_UI4 => write!(f, "Variant(&UI4({:?}))", self.borrow::<&types::UI4>()),
			vt::VT_BYREF_INT => write!(f, "Variant(&Int({:?}))", self.borrow::<&types::Int>()),
			vt::VT_BYREF_UINT => write!(f, "Variant(&UInt({:?}))", self.borrow::<&types::UInt>()),
			// vt::VT_BYREF_RECORD => f.field("&record", ),
			_ => write!(f, "Variant({:?})", self.tag()),
		}
	}
}

//----------------------------------------------------------------

/// Legal `Variant` type.
pub unsafe trait Type<'a>: 'a + Sized {
	/// Returns the type's variant tag.
	fn variant_tag() -> Tag;

	/// Raw type stored in the `Variant`.
	type Raw: Sized;
	/// Borrows the raw type.
	unsafe fn variant_raw_borrow<'s>(var: &'s Variant<'a>) -> &'s Self::Raw {
		mem::transmute(&var.0.data1)
	}
	/// Extracts the raw type as owned.
	unsafe fn variant_raw_into(var: Variant<'a>) -> Self::Raw {
		let val = mem::transmute_copy(&var.0.data1);
		mem::forget(var);
		val
	}
	/// Creates a `Variant` from the raw type.
	unsafe fn variant_raw_from(raw: Self::Raw) -> Variant<'a> {
		let mut var = Variant::init();
		ptr::write(&mut var.0.data0 as *mut _ as *mut Tag, Self::variant_tag());
		ptr::write(&mut var.0.data1 as *mut _ as *mut Self::Raw, raw);
		var
	}

	/// Idiomatic Rust type when the variant is borrowed.
	type Borrowed: ?Sized;
	/// Idiomatic Rust type when the variant is consumed.
	type Owned: 'a + Sized;
	/// Borrows the variant as type.
	unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s Self::Borrowed;
	/// Consumes the variant as type.
	unsafe fn variant_into(var: Variant<'a>) -> Self::Owned;
	/// Creates a variant from type.
	unsafe fn variant_from(self) -> Variant<'a>;
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn units() {
		let empty = Variant::new();
		assert_eq!(empty.into::<types::Empty>(), types::Empty);
	}
}
