#![no_std]

use core::mem::size_of;
use core::ptr::NonNull;

#[inline(always)]
pub fn pointdist<T>(start: *const T, end: *const T) -> usize
{
	let size = size_of::<T>();
	if size == 0
	{
		(end as usize).wrapping_sub(start as usize)
	}
	else
	{
		(end as usize - start as usize) / size
	}
}

pub trait PointExt : Copy
{
	unsafe fn offset(self, i: isize) -> Self;
	unsafe fn add(self, i: usize) -> Self
	{
		self.offset(i as isize)
	}
	unsafe fn sub(self, i: usize) -> Self
	{
		self.offset((i as isize).wrapping_neg())
	}
	#[inline(always)]
	unsafe fn preinc(&mut self) -> Self
	{
		*self = self.offset(1);
		*self
	}
	#[inline(always)]
	unsafe fn postinc(&mut self) -> Self
	{
		let curr = *self;
		*self = self.offset(1);
		curr
	}
	#[inline(always)]
	unsafe fn predec(&mut self) -> Self
	{
		*self = self.offset(-1);
		*self
	}
	#[inline(always)]
	unsafe fn postdec(&mut self) -> Self
	{
		let curr = *self;
		*self = self.offset(-1);
		curr
	}
	#[inline(always)]
	unsafe fn inc(&mut self)
	{
		*self = self.offset(1);
	}
	#[inline(always)]
	unsafe fn dec(&mut self)
	{
		*self = self.offset(-1);
	}
	#[inline(always)]
	unsafe fn strideoffset(self, s: isize, index: usize) -> Self
	{
		self.offset(s * index as isize)
	}
}

impl<T> PointExt for *const T
{
	#[inline(always)]
	unsafe fn offset(self, i: isize) -> Self
	{
		self.offset(i)
	}
	#[inline]
	unsafe fn add(self, i: usize) -> Self
	{
		self.add(i)
	}
	#[inline]
	unsafe fn sub(self, i: usize) -> Self
	{
		self.sub(i)
	}
}

impl<T> PointExt for *mut T
{
	#[inline(always)]
	unsafe fn offset(self, i: isize) -> Self
	{
		self.offset(i)
	}
	#[inline]
	unsafe fn add(self, i: usize) -> Self
	{
		self.add(i)
	}
	#[inline]
	unsafe fn sub(self, i: usize) -> Self
	{
		self.sub(i)
	}
}

impl<T> PointExt for NonNull<T>
{
	#[inline(always)]
	unsafe fn offset(self, i: isize) -> Self
	{
		NonNull::new_unchecked(self.as_ptr().offset(i))
	}
}
