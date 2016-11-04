
use ::std::{fmt, ops, slice, mem};

use super::ElemType;

//----------------------------------------------------------------

#[repr(C)]
pub struct ArrSlice<T: ElemType>([T::Raw]);
impl<T: ElemType> ArrSlice<T> {
	pub fn iter<'s>(&'s self) -> ArrIter<'s, T> {
		ArrIter(self.0.iter())
	}
}

use ::{IntoInner, FromInner, AsInner, AsInnerMut};
impl<'a, T: 'a + ElemType> IntoInner<&'a [T::Raw]> for &'a ArrSlice<T> {
	fn into_inner(self) -> &'a [T::Raw] {
		&self.0
	}
}
impl<'a, T: 'a + ElemType> FromInner<&'a [T::Raw]> for &'a ArrSlice<T> {
	unsafe fn from_inner(raw: &'a [T::Raw]) -> &'a ArrSlice<T> {
		mem::transmute(raw)
	}
}
impl<T: ElemType> AsInner<[T::Raw]> for ArrSlice<T> {
	fn as_inner(&self) -> &[T::Raw] {
		&self.0
	}
}
impl<T: ElemType> AsInnerMut<[T::Raw]> for ArrSlice<T> {
	unsafe fn as_inner_mut(&mut self) -> &mut [T::Raw] {
		&mut self.0
	}
}

impl<T: ElemType> ops::Index<usize> for ArrSlice<T> {
	type Output = T::Ref;
	fn index(&self, index: usize) -> &T::Ref {
		T::elem_ref(&self.0[index])
	}
}
impl<T: ElemType> ops::Index<ops::Range<usize>> for ArrSlice<T> {
	type Output = ArrSlice<T>;
	fn index(&self, index: ops::Range<usize>) -> &ArrSlice<T> {
		unsafe { <_ as FromInner<_>>::from_inner(&self.0[index]) }
	}
}
impl<T: ElemType> ops::Index<ops::RangeFrom<usize>> for ArrSlice<T> {
	type Output = ArrSlice<T>;
	fn index(&self, index: ops::RangeFrom<usize>) -> &ArrSlice<T> {
		unsafe { <_ as FromInner<_>>::from_inner(&self.0[index]) }
	}
}
impl<T: ElemType> ops::Index<ops::RangeTo<usize>> for ArrSlice<T> {
	type Output = ArrSlice<T>;
	fn index(&self, index: ops::RangeTo<usize>) -> &ArrSlice<T> {
		unsafe { <_ as FromInner<_>>::from_inner(&self.0[index]) }
	}
}
impl<T: ElemType> ops::Index<ops::RangeFull> for ArrSlice<T> {
	type Output = ArrSlice<T>;
	fn index(&self, index: ops::RangeFull) -> &ArrSlice<T> {
		unsafe { <_ as FromInner<_>>::from_inner(&self.0[index]) }
	}
}

impl<T: ElemType> fmt::Debug for ArrSlice<T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_list()
			.entries(self.iter())
			.finish()
	}
}

//----------------------------------------------------------------

#[derive(Clone)]
pub struct ArrIter<'a, T: 'a + ElemType>(slice::Iter<'a, T::Raw>);
impl<'a, T: 'a + ElemType> ArrIter<'a, T> {
	pub fn as_slice(&self) -> &'a ArrSlice<T> {
		unsafe { <_ as FromInner<_>>::from_inner(self.0.as_slice()) }
	}
}
impl<'a, T: 'a + ElemType> Iterator for ArrIter<'a, T> {
	type Item = &'a T::Ref;
	fn next(&mut self) -> Option<&'a T::Ref> {
		self.0.next().map(|raw| T::elem_ref(raw))
	}
}
impl<'a, T: 'a + ElemType> DoubleEndedIterator for ArrIter<'a, T> {
	fn next_back(&mut self) -> Option<&'a T::Ref> {
		self.0.next_back().map(|raw| T::elem_ref(raw))
	}
}
impl<'a, T: 'a + ElemType> ExactSizeIterator for ArrIter<'a, T> {
	fn len(&self) -> usize {
		self.0.len()
	}
}
impl<'a, T: 'a + ElemType> fmt::Debug for ArrIter<'a, T> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		f.debug_tuple("ArrIter")
			.field(&self.as_slice())
			.finish()
	}
}
