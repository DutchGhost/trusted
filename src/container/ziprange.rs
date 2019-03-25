use crate::fundemental::index::Index;
use crate::fundemental::id::Id;
use crate::fundemental::range::Range;

/// An Iterator that zip's range's from 2 different containers.
pub struct ZipRange<'lhs, 'rhs> {
    _lhs_id: Id<'lhs>,
    _rhs_id: Id<'rhs>,
    start: usize,
    end: usize
}

impl <'lhs, 'rhs> ZipRange<'lhs, 'rhs> {
    #[inline(always)]
    pub(crate) const unsafe fn new(start: usize, end: usize) -> ZipRange<'lhs, 'rhs> {
        ZipRange {
            _lhs_id: Id::new(),
            _rhs_id: Id::new(),
            start,
            end
        }
    }

    #[inline(always)]
    pub const fn into_ranges(self) -> (Range<'lhs>, Range<'rhs>) {
        unsafe {
            (
                Range::from(self.start, self.end),
                Range::from(self.start, self.end)
            )
        }
    }
}

impl <'lhs, 'rhs> Iterator for ZipRange<'lhs, 'rhs> {
    type Item = (Index<'lhs>, Index<'rhs>);

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let idx = self.start;
            self.start += 1;
            unsafe { Some((Index::new(idx), Index::new(idx))) }
        } else {
            None
        }
    }
}

impl<'lhs, 'rhs> DoubleEndedIterator for ZipRange<'lhs, 'rhs> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;

            unsafe { Some((Index::new(self.end), Index::new(self.end))) }
        } else {
            None
        }
    }
}

impl<'lhs, 'rhs> std::iter::ExactSizeIterator for ZipRange<'lhs, 'rhs> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}
