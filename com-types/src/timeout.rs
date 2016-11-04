/*!
Timeout data type.
*/

use ::winapi::LONG;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Timeout(LONG);

impl Timeout {
}

impl_inner_newtype!(Timeout: LONG);

pub const INFINITE: Timeout = Timeout(-1);
pub const NO_WAIT: Timeout = Timeout(0);
