/*!
Decomposing `HResult`.
*/

use ::std::fmt;

// pub mod facility;
pub mod code;

/// Result severity.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Severity {
	Success,
	Error,
}

pub type SCode = HResult;

/// System result.
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct HResult(pub u32);
impl From<i32> for HResult {
	fn from(hr: i32) -> HResult {
		HResult(hr as u32)
	}
}
impl From<HResult> for i32 {
	fn from(hr: HResult) -> i32 {
		hr.0 as i32
	}
}
impl HResult {
	/// Returns if the result has succeeded.
	pub fn succeeded(self) -> bool {
		self.0 & 0x80000000 == 0
	}
	/// Returns if the result has failed.
	pub fn failed(self) -> bool {
		self.0 & 0x80000000 != 0
	}
	/// Returns the severity of the result.
	pub fn severity(self) -> Severity {
		if self.0 & 0x80000000 != 0 {
			Severity::Error
		}
		else {
			Severity::Success
		}
	}
	/// Returns the facility of the result.
	pub fn facility(self) -> u32 {
		(self.0 >> 16) % (1 << 11)
	}
	/// Returns the error code of the result.
	pub fn code(self) -> u32 {
		(self.0) % (1 << 16)
	}
	/// Returns the result as a `Result`.
	///
	/// Returns `Ok(val)` if the result has succeeded, otherwise `Err(hr)` if the result has failed.
	///
	/// On success the result is discarded.
	pub fn result<R, T>(hr: R, val: T) -> Result<T, HResult> where R: Into<HResult> {
		let hr = hr.into();
		match hr.severity() {
			Severity::Success => Ok(val),
			Severity::Error => Err(hr),
		}
	}
	/// Returns the result as a `Result`.
	///
	/// Calls `f` if the result has succeeded and returns the `Ok` value, otherwise `Err(hr)` if the result has failed.
	///
	/// On success the result is discarded.
	pub fn result_of<R, T, F>(hr: R, f: F) -> Result<T, HResult>
		where R: Into<HResult>,
		      F: FnOnce() -> T
	{
		let hr = hr.into();
		match hr.severity() {
			Severity::Success => Ok(f()),
			Severity::Error => Err(hr),
		}
	}
}

//----------------------------------------------------------------
// Formatter

impl fmt::Display for HResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#010X}", self.0)
	}
}
impl fmt::Debug for HResult {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{:#010X}", self.0)
	}
}

//----------------------------------------------------------------

#[cfg(test)]
mod tests {
	use super::*;
	#[test]
	fn units() {
		assert_eq!(format!("{}", code::E_INVALIDARG), "0x80070057");
	}
}
