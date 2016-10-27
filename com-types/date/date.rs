/*!
Date data type. Incomplete.
*/

extern crate winapi;
#[macro_use]
extern crate inner;

use winapi::DATE;

/// Date data type.
///
/// See [MSDN](https://msdn.microsoft.com/en-us/library/82ab7w69.aspx).
#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Date(pub DATE);
impl From<DATE> for Date {
	fn from(date: DATE) -> Date {
		Date(date)
	}
}
impl From<Date> for DATE {
	fn from(date: Date) -> DATE {
		date.0
	}
}
impl AsRef<DATE> for Date {
	fn as_ref(&self) -> &DATE {
		&self.0
	}
}
impl AsMut<DATE> for Date {
	fn as_mut(&mut self) -> &mut DATE {
		&mut self.0
	}
}

//----------------------------------------------------------------

impl_inner_newtype!(Date: DATE);
