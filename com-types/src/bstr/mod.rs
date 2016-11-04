/*!
Basic Strings ‘`BSTR`’ for COM.

See [MSDN](https://msdn.microsoft.com/en-us/library/windows/desktop/ms221069.aspx)
and [Eric’s Complete Guide To BSTR Semantics](https://blogs.msdn.microsoft.com/ericlippert/2003/09/12/erics-complete-guide-to-bstr-semantics/).

*/

mod array_traits;

mod barray;
mod bstr;
mod bstring;

pub use self::bstr::{AsRawBStr, BStr, NullBStr};
pub use self::bstring::{BString, IntoBString};
pub use self::barray::{BArray};
