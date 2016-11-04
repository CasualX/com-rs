/*!
COM bool.
*/

//----------------------------------------------------------------

/// A 16-bit Boolean value.
///
/// A value of 0xFFFF (all bits 1) indicates true; a value of 0 (all bits 0) indicates false. No other values are valid.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct Bool(u16);
impl From<bool> for Bool {
	fn from(val: bool) -> Bool {
		Bool(if val { !0 } else { 0 })
	}
}
impl From<Bool> for bool {
	fn from(val: Bool) -> bool {
		val.0 != 0
	}
}

impl Default for Bool {
	fn default() -> Bool {
		FALSE
	}
}

//----------------------------------------------------------------

impl_inner_newtype!(Bool: u16);

//----------------------------------------------------------------

pub const TRUE: Bool = Bool(!0);
pub const FALSE: Bool = Bool(0);

//----------------------------------------------------------------

use ::std::fmt;

impl fmt::Debug for Bool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("Bool").field(&bool::from(*self)).finish()
	}
}
impl fmt::Display for Bool {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		bool::from(*self).fmt(f)
	}
}
