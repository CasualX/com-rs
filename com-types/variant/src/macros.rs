/*!
`Type` implementation macros.

Most of types implementing `Type` follow one of few patterns.

To avoid too much duplication these patterns are implemented in the following macros.

Use these macros only in the `types` module or things may break.
*/

macro_rules! impl_type_unit {
	($vt:ident, $rust_type:path) => {
		unsafe impl<'a> Type<'a> for $rust_type {
			fn variant_tag() -> Tag { vt::$vt }
			type Raw = ();
			type Borrowed = $rust_type;
			type Owned = $rust_type;
			unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s $rust_type {
				::std::mem::transmute(Self::variant_raw_borrow(var))
			}
			unsafe fn variant_into(_: Variant<'a>) -> $rust_type {
				$rust_type
			}
			unsafe fn variant_from(self) -> Variant<'a> {
				let mut var = Variant::new();
				::std::ptr::write(&mut var.0.data0 as *mut _ as *mut Tag, Self::variant_tag());
				var
			}
		}
	}
}

/// Implement `Type` for `#[repr(C)]` tuple structs wrapping the native type. No special `Drop` handling.
macro_rules! impl_type_reprc {
	($vt:ident, $raw_type:ty, $rust_type:ty) => {
		unsafe impl<'a> Type<'a> for $rust_type {
			fn variant_tag() -> Tag { vt::$vt }
			type Raw = $raw_type;
			type Borrowed = $rust_type;
			type Owned = $rust_type;
			unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s $rust_type {
				&*(Self::variant_raw_borrow(var) as *const _ as *const $rust_type)
			}
			unsafe fn variant_into(var: Variant<'a>) -> $rust_type {
				use ::inner::FromInner;
				<$rust_type as FromInner<$raw_type>>::from_inner(Self::variant_raw_into(var))
			}
			unsafe fn variant_from(self) -> Variant<'a> {
				use ::inner::IntoInner;
				Self::variant_raw_from(self.into_inner())
			}
		}
	}
}

macro_rules! impl_type_ref_reprc {
	($rust_type:ty) => {
		unsafe impl<'a> Type<'a> for &'a $rust_type {
			fn variant_tag() -> Tag {
				Tag(vt::VT_BYREF | <$rust_type as Type>::variant_tag().0)
			}
			type Raw = *const <$rust_type as Type<'a>>::Raw;
			type Borrowed = $rust_type;
			type Owned = &'a $rust_type;
			unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s $rust_type {
				&*(*Self::variant_raw_borrow(var) as *const $rust_type)
			}
			unsafe fn variant_into(var: Variant<'a>) -> &'a $rust_type {
				&*(Self::variant_raw_into(var) as *const $rust_type)
			}
			unsafe fn variant_from(self) -> Variant<'a> {
				Self::variant_raw_from(self as *const _ as *const <$rust_type as Type>::Raw)
			}
		}
	}
}

/// Implement `Type` for native types. No special `Drop` handling.
macro_rules! impl_type_native {
	($vt:ident, $ty:ty) => {
		unsafe impl<'a> Type<'a> for $ty {
			fn variant_tag() -> Tag { vt::$vt }
			type Raw = $ty;
			type Borrowed = $ty;
			type Owned = $ty;
			unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s $ty {
				Self::variant_raw_borrow(var)
			}
			unsafe fn variant_into(var: Variant<'a>) -> $ty {
				Self::variant_raw_into(var)
			}
			unsafe fn variant_from(self) -> Variant<'a> {
				Self::variant_raw_from(self)
			}
		}
	}
}

macro_rules! impl_type_ref_native {
	($ty:ty) => {
		unsafe impl<'a> Type<'a> for &'a $ty {
			fn variant_tag() -> Tag {
				Tag(vt::VT_BYREF | <$ty as Type>::variant_tag().0)
			}
			type Raw = *const $ty;
			type Borrowed = $ty;
			type Owned = &'a $ty;
			unsafe fn variant_borrow<'s>(var: &'s Variant<'a>) -> &'s $ty {
				&**Self::variant_raw_borrow(var)
			}
			unsafe fn variant_into(var: Variant<'a>) -> &'a $ty {
				&*Self::variant_raw_into(var)
			}
			unsafe fn variant_from(self) -> Variant<'a> {
				Self::variant_raw_from(self)
			}
		}
	}
}
