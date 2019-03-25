use super::traits::*;
use crate::fundemental::proof::*;
use crate::fundemental::{id::Id, index::Index, range::Range};

pub struct Container<'id, C> {
    _id: Id<'id>,
    container: C,
}

impl<'id, C, T> Container<'id, C>
where
    C: ContainerTrait<Item = T>,
{
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.container.base_len()
    }

    #[inline(always)]
    pub fn range(&self) -> Range<'id> {
        unsafe { Range::from(0, self.len()) }
    }

    #[inline(always)]
    pub fn split_at<P>(&self, index: Index<'id, P>) -> (Range<'id>, Range<'id, P>) {
        unsafe {
            (
                Range::from(0, index.index),
                Range::from_any(index.index, self.len()),
            )
        }
    }

    #[inline(always)]
    pub fn swap(&mut self, a: Index<'id>, b: Index<'id>)
    where
        C: GetUncheckedMut,
    {
        use std::ptr;

        unsafe {
            let self_mut = self as *mut Self;
            let pa: *mut _ = &mut (*self_mut)[a];
            let pb: *mut _ = &mut (*self_mut)[b];

            ptr::swap(pa, pb);
        }
    }

    #[inline]
    pub fn scan_from_rev<'b, F>(&'b self, index: Index<'id>, mut f: F) -> Range<'id, NonEmpty>
    where
        F: FnMut(&'b T) -> bool,
        T: 'b,
        C: Contiguous,
    {
        unsafe {
            let mut start = index;
            for elt in self[..index].iter().rev() {
                if !f(elt) {
                    break;
                }
                start.index -= 1;
            }
            Range::from_ne(start.index, index.index + 1)
        }
    }

    #[inline]
    pub fn rotate1_up(&mut self, r: Range<'id, NonEmpty>)
    where
        C: Contiguous + GetUncheckedMut,
    {
        use std::{mem, ptr};

        if r.first() != r.last() {
            unsafe {
                let last_ptr = &self[r.last()] as *const C::Item;
                let first_ptr = &mut self[r.first()] as *mut C::Item;
                let tmp = ptr::read(last_ptr);
                ptr::copy(first_ptr, first_ptr.offset(1), r.len() - 1);
                ptr::copy_nonoverlapping(&tmp, first_ptr, 1);
                mem::forget(tmp);
            }
        }
    }
}

use std::ops;

impl<'id, C> ops::Index<Index<'id>> for Container<'id, C>
where
    C: GetUnchecked,
{
    type Output = C::Item;

    #[inline(always)]
    fn index(&self, index: Index<'id>) -> &Self::Output {
        unsafe { self.container.unchecked(index.index) }
    }
}

impl<'id, C> ops::IndexMut<Index<'id>> for Container<'id, C>
where
    C: GetUncheckedMut,
{
    #[inline(always)]
    fn index_mut(&mut self, index: Index<'id>) -> &mut Self::Output {
        unsafe { self.container.unchecked_mut(index.index) }
    }
}

impl<'id, T, C, P> ops::Index<Range<'id, P>> for Container<'id, C>
where
    C: Contiguous<Item = T>,
{
    type Output = [T];

    #[inline(always)]
    fn index(&self, r: Range<'id, P>) -> &Self::Output {
        use std::slice;
        unsafe { slice::from_raw_parts(self.container.begin().offset(r.start() as isize), r.len()) }
    }
}

impl<'id, C, P> ops::IndexMut<Range<'id, P>> for Container<'id, C>
where
    C: ContiguousMut,
{
    #[inline(always)]
    fn index_mut(&mut self, r: Range<'id, P>) -> &mut Self::Output {
        use std::slice;
        unsafe {
            slice::from_raw_parts_mut(
                self.container.begin_mut().offset(r.start() as isize),
                r.len(),
            )
        }
    }
}

impl<'id, T, P, C> ops::Index<ops::RangeFrom<Index<'id, P>>> for Container<'id, C>
where
    C: Contiguous<Item = T>,
{
    type Output = [T];

    #[inline(always)]
    fn index(&self, r: ops::RangeFrom<Index<'id, P>>) -> &Self::Output {
        use std::slice;
        let i = r.start.index;

        unsafe { slice::from_raw_parts(self.container.begin().offset(i as isize), self.len() - i) }
    }
}

impl<'id, P, C> ops::IndexMut<ops::RangeFrom<Index<'id, P>>> for Container<'id, C>
where
    C: ContiguousMut,
{
    #[inline(always)]
    fn index_mut(&mut self, r: ops::RangeFrom<Index<'id, P>>) -> &mut Self::Output {
        use std::slice;
        let i = r.start.index;

        unsafe {
            slice::from_raw_parts_mut(
                self.container.begin_mut().offset(i as isize),
                self.len() - i,
            )
        }
    }
}

impl<'id, T, P, C> ops::Index<ops::RangeTo<Index<'id, P>>> for Container<'id, C>
where
    C: Contiguous<Item = T>,
{
    type Output = [T];

    #[inline(always)]
    fn index(&self, r: ops::RangeTo<Index<'id, P>>) -> &Self::Output {
        use std::slice;
        let i = r.end.index;

        unsafe { slice::from_raw_parts(self.container.begin(), i) }
    }
}

impl<'id, P, C> ops::IndexMut<ops::RangeTo<Index<'id, P>>> for Container<'id, C>
where
    C: ContiguousMut,
{
    #[inline(always)]
    fn index_mut(&mut self, r: ops::RangeTo<Index<'id, P>>) -> &mut Self::Output {
        use std::slice;
        let i = r.end.index;

        unsafe { slice::from_raw_parts_mut(self.container.begin_mut(), i) }
    }
}

impl<'id, T, C> ops::Index<ops::RangeFull> for Container<'id, C>
where
    C: Contiguous<Item = T>,
{
    type Output = [T];

    #[inline(always)]
    fn index(&self, _: ops::RangeFull) -> &Self::Output {
        self.container.as_slice()
    }
}

impl<'id, C> ops::IndexMut<ops::RangeFull> for Container<'id, C>
where
    C: ContiguousMut,
{
    #[inline(always)]
    fn index_mut(&mut self, _: ops::RangeFull) -> &mut Self::Output {
        self.container.as_mut_slice()
    }
}

pub fn scope<C, F, Out>(container: C, f: F) -> Out
where
    F: for<'id> FnOnce(Container<'id, C>) -> Out,
    C: ContainerTrait,
{
    f(Container {
        _id: Id::new(),
        container,
    })
}
