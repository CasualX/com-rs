/*!
Basic Strings ‘`BSTR`’ for COM.

See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221069.aspx)
and [Eric’s Complete Guide To BSTR Semantics](https://blogs.msdn.microsoft.com/ericlippert/2003/09/12/erics-complete-guide-to-bstr-semantics/).

*/

#![cfg(windows)]

extern crate oleaut32;
extern crate winapi;

mod bstr;
mod bstring;

mod array_traits;
mod barray;

pub use bstr::{AsRawBStr, BStr, NullBStr};
pub use bstring::{BString, IntoBString};
pub use barray::{BArray};
