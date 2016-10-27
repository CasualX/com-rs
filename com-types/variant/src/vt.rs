/*!
Variant type tags.
*/

use super::Tag;

pub const VT_EMPTY: Tag = Tag(0);
pub const VT_NULL: Tag = Tag(1);
pub const VT_I2: Tag = Tag(2);
pub const VT_I4: Tag = Tag(3);
pub const VT_R4: Tag = Tag(4);
pub const VT_R8: Tag = Tag(5);
pub const VT_CY: Tag = Tag(6);
pub const VT_DATE: Tag = Tag(7);
pub const VT_BSTR: Tag = Tag(8);
pub const VT_DISPATCH: Tag = Tag(9);
pub const VT_ERROR: Tag = Tag(10);
pub const VT_BOOL: Tag = Tag(11);
pub const VT_UNKNOWN: Tag = Tag(13);
pub const VT_DECIMAL: Tag = Tag(14);
pub const VT_I1: Tag = Tag(16);
pub const VT_UI1: Tag = Tag(17);
pub const VT_UI2: Tag = Tag(18);
pub const VT_UI4: Tag = Tag(19);
pub const VT_INT: Tag = Tag(22);
pub const VT_UINT: Tag = Tag(23);
pub const VT_RECORD: Tag = Tag(36);

pub const VT_BYREF: u16 = 0x4000;

pub const VT_BYREF_I2: Tag = Tag(VT_BYREF | 2);
pub const VT_BYREF_I4: Tag = Tag(VT_BYREF | 3);
pub const VT_BYREF_R4: Tag = Tag(VT_BYREF | 4);
pub const VT_BYREF_R8: Tag = Tag(VT_BYREF | 5);
pub const VT_BYREF_CY: Tag = Tag(VT_BYREF | 6);
pub const VT_BYREF_DATE: Tag = Tag(VT_BYREF | 7);
pub const VT_BYREF_BSTR: Tag = Tag(VT_BYREF | 8);
pub const VT_BYREF_DISPATCH: Tag = Tag(VT_BYREF | 9);
pub const VT_BYREF_ERROR: Tag = Tag(VT_BYREF | 10);
pub const VT_BYREF_BOOL: Tag = Tag(VT_BYREF | 11);
pub const VT_BYREF_VARIANT: Tag = Tag(VT_BYREF | 12);
pub const VT_BYREF_UNKNOWN: Tag = Tag(VT_BYREF | 13);
pub const VT_BYREF_DECIMAL: Tag = Tag(VT_BYREF | 14);
pub const VT_BYREF_I1: Tag = Tag(VT_BYREF | 16);
pub const VT_BYREF_UI1: Tag = Tag(VT_BYREF | 17);
pub const VT_BYREF_UI2: Tag = Tag(VT_BYREF | 18);
pub const VT_BYREF_UI4: Tag = Tag(VT_BYREF | 19);
pub const VT_BYREF_INT: Tag = Tag(VT_BYREF | 22);
pub const VT_BYREF_UINT: Tag = Tag(VT_BYREF | 23);
pub const VT_BYREF_RECORD: Tag = Tag(VT_BYREF | 36);
