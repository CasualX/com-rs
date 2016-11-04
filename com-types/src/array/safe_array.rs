
use ::std::{ops, fmt};

use ::winapi::{SAFEARRAY};

use super::sys::{SafeArrayDestroy};

use super::safe_arr::SafeArr;

pub struct SafeArray(*mut SAFEARRAY);
impl Drop for SafeArray {
	fn drop(&mut self) {
		unsafe {
			let _ = SafeArrayDestroy(self.0);
		}
	}
}

//----------------------------------------------------------------
// AsRef, Deref

impl AsRef<SafeArr> for SafeArray {
	fn as_ref(&self) -> &SafeArr {
		unsafe { SafeArr::from_raw(self.0) }
	}
}
impl ops::Deref for SafeArray {
	type Target = SafeArr;
	fn deref(&self) -> &SafeArr {
		unsafe { SafeArr::from_raw(self.0) }
	}
}

//----------------------------------------------------------------
// Debug

impl fmt::Debug for SafeArray {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		(**self).fmt(f)
	}
}

//----------------------------------------------------------------
