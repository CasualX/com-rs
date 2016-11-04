/*!
*/

use ::winapi::DECIMAL;

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Decimal(DECIMAL);

//----------------------------------------------------------------

impl_inner_newtype!(Decimal: DECIMAL);
