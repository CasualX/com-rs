/*!
Common `HResult` error codes.
*/

use super::HResult;

pub const S_OK: HResult = HResult(0x00000000);
pub const E_NOTIMPL: HResult = HResult(0x80004001);
pub const E_NOINTERFACE: HResult = HResult(0x80004002);
pub const E_POINTER: HResult = HResult(0x80004003);
pub const E_ABORT: HResult = HResult(0x80004004);
pub const E_FAIL: HResult = HResult(0x80004005);
pub const E_UNEXPECTED: HResult = HResult(0x8000FFFF);
pub const E_ACCESSDENIED: HResult = HResult(0x80070005);
pub const E_HANDLE: HResult = HResult(0x80070006);
pub const E_OUTOFMEMORY: HResult = HResult(0x8007000E);
pub const E_INVALIDARG: HResult = HResult(0x80070057);
