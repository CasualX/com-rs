/*!
Currency data type. Incomplete.
*/

use ::std::{fmt, mem};

use ::winapi::CY;

//----------------------------------------------------------------

/// Currency data type.
///
/// See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221223.aspx).
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Currency(CY);
impl From<i64> for Currency {
	fn from(cy: i64) -> Currency {
		Currency(unsafe { mem::transmute(cy) })
	}
}
impl From<Currency> for i64 {
	fn from(cy: Currency) -> i64 {
		unsafe { mem::transmute(cy.0) }
	}
}
impl AsRef<i64> for Currency {
	fn as_ref(&self) -> &i64 {
		unsafe { mem::transmute(&self.0) }
	}
}
impl AsMut<i64> for Currency {
	fn as_mut(&mut self) -> &mut i64 {
		unsafe { mem::transmute(&mut self.0) }
	}
}

//----------------------------------------------------------------

impl_inner_newtype!(Currency: CY);

//----------------------------------------------------------------

impl fmt::Debug for Currency {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		let num: i64 = (*self).into();
		let int = num / 10000;
		let fract = num.abs() % 10000;
		write!(f, "{}.{:>04}", int, fract)
	}
}
