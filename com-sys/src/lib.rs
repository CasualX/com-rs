/*!
*/

#![cfg(windows)]

extern crate winapi;

/// Re-export so that the macros can find them.
#[doc(hidden)]
pub use winapi::{HRESULT, GUID, LPVOID};

#[macro_use]
pub mod macros;

mod traits;
pub use traits::*;

pub mod unknown;
