/*!
*/

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timeout(i32);

impl From<i32> for Timeout {
	fn from(timeout: i32) -> Timeout {
		Timeout(timeout)
	}
}
impl From<Timeout> for i32 {
	fn from(timeout: Timeout) -> i32 {
		timeout.0
	}
}

pub const INFINITE: Timeout = Timeout(-1);
pub const NO_WAIT: Timeout = Timeout(0);
