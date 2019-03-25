use super::{
    id::Id,
    index::Index,
    proof::{NonEmpty, ProofAdd, Unknown},
};

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Range<'id, Proof = Unknown> {
    _id: Id<'id>,
    start: usize,
    end: usize,
    proof: PhantomData<Proof>,
}

impl<'id> Range<'id> {
    /// Creates a new Unknow range from `start` and `end`.
    /// This function is marked unsafe,
    /// because it can not be proved `start` and `end` are a valid
    /// range of the container.
    #[inline]
    pub const unsafe fn from(start: usize, end: usize) -> Range<'id> {
        Range {
            _id: Id::new(),
            start,
            end,
            proof: PhantomData,
        }
    }
}

impl<'id> Range<'id, NonEmpty> {
    /// Creates a new NonEmpty range from `start` and `end`.
    /// This function is marked unsafe,
    /// because it can not be proved `start` and `end` are a valid
    /// range of the container.
    #[inline]
    pub const unsafe fn from_ne(start: usize, end: usize) -> Range<'id, NonEmpty> {
        Range {
            _id: Id::new(),
            start,
            end,
            proof: PhantomData,
        }
    }
}

impl<'id, P> Range<'id, P> {
    /// Creates a new range from `start` and `end`.
    /// This function is marked unsafe,
    /// because it can not be proved `start` and `end` are a valid
    /// range of the container.
    #[inline]
    pub const unsafe fn from_any(start: usize, end: usize) -> Range<'id, P> {
        Range {
            _id: Id::new(),
            start,
            end,
            proof: PhantomData,
        }
    }
}

impl<'id, P> Copy for Range<'id, P> {}

impl<'id, P> Clone for Range<'id, P> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'id, P> Range<'id, P> {
    #[inline]
    pub fn nonempty(&self) -> Option<Range<'id, NonEmpty>> {
        if !self.is_empty() {
            unsafe { Some(std::mem::transmute(*self)) }
        } else {
            None
        }
    }
    /// Returns the length of the range.
    #[inline]
    pub const fn len(&self) -> usize {
        self.end - self.start
    }

    /// Returns `true` if the range is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.start >= self.end
    }

    /// Returns the start index of the range.
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// Returns the end index of the range.
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Split the range in half, with the
    /// upper middle index landing in the latter half.
    /// Proof of length `P` transfers to the latter half.
    #[inline]
    pub const fn split_in_half(&self) -> (Range<'id>, Range<'id, P>) {
        let mid = (self.end - self.start) / 2 + self.start;

        unsafe { (Range::from(self.start, mid), Range::from_any(mid, self.end)) }
    }

    #[inline]
    pub fn split_at(&self, index: usize) -> (Range<'id>, Range<'id>, bool) {
        let mid = if index > self.len() {
            self.end
        } else {
            self.start + index
        };

        unsafe {
            (
                Range::from(self.start, mid),
                Range::from(mid, self.end),
                index <= self.len(),
            )
        }
    }

    #[inline]
    pub fn contains(&self, index: usize) -> Option<Index<'id, P>> {
        unsafe {
            if index >= self.start && index < self.end {
                Some(Index::new(index))
            } else {
                None
            }
        }
    }

    #[inline]
    pub const fn join<Q>(
        &self,
        other: Range<'id, Q>,
    ) -> Result<Range<'id, <(P, Q) as ProofAdd>::Sum>, ()>
    where
        (P, Q): ProofAdd,
    {
        unsafe {
            [Ok(Range::from_any(self.start, other.end)), Err(())]
                [(self.end != other.start) as usize]
        }
        // if self.end == other.start {
        //     unsafe {
        //         Ok(Range::from_any(self.start, other.end))
        //     }
        // } else {
        //     Err(())
        // }
    }
}

impl<'id, P> Range<'id, P> {
    #[inline]
    pub const unsafe fn nonempty_unchecked(&self) -> Range<'id, NonEmpty> {
        Range::from_any(self.start, self.end)
    }

    pub const fn first(&self) -> Index<'id, P> {
        unsafe { Index::new(self.start) }
    }

    pub const fn upper_middle(&self) -> Index<'id, P> {
        let mid = self.len() / 2 + self.start;

        unsafe { Index::new(mid) }
    }

    /// Split the range at `index`. Proof of length `P` transfers to the latter end.
    #[inline]
    pub const fn split_index(&self, index: Index<'id>) -> (Range<'id>, Range<'id, P>) {
        unsafe {
            (
                Range::from(self.start, index.integer()),
                Range::from_any(index.integer(), self.end),
            )
        }
    }
}

impl<'id> Range<'id, NonEmpty> {
    #[inline]
    pub const unsafe fn unsafe_split_index(
        &self,
        index: Index<'id>,
    ) -> (Range<'id, NonEmpty>, Range<'id, NonEmpty>) {
        (
            Range::from_ne(self.start, index.integer()),
            Range::from_ne(index.integer(), self.end),
        )
    }

    pub const fn last(&self) -> Index<'id> {
        unsafe { Index::new(self.end - 1) }
    }

    pub const fn tail(self) -> Range<'id> {
        unsafe { Range::from(self.start + 1, self.end) }
    }

    pub fn advance_back(&mut self) -> bool {
        let mut next = *self;
        next.end -= 1;
        if next.start < next.end {
            *self = next;
            true
        } else {
            false
        }
    }

    pub fn advance(&mut self) -> bool {
        let mut next = *self;
        next.start += 1;

        if next.start < next.end {
            *self = next;
            true
        } else {
            false
        }
    }
}

impl<'id, P> IntoIterator for Range<'id, P> {
    type Item = Index<'id>;
    type IntoIter = RangeIter<'id>;

    #[inline(always)]
    fn into_iter(self) -> Self::IntoIter {
        RangeIter {
            _id: self._id,
            start: self.start,
            end: self.end,
        }
    }
}

pub struct RangeIter<'id> {
    _id: Id<'id>,
    start: usize,
    end: usize,
}

impl<'id> Iterator for RangeIter<'id> {
    type Item = Index<'id>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let idx = self.start;
            self.start += 1;
            unsafe { Some(Index::new(idx)) }
        } else {
            None
        }
    }
}

impl<'id> DoubleEndedIterator for RangeIter<'id> {
    #[inline(always)]
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            self.end -= 1;

            unsafe { Some(Index::new(self.end)) }
        } else {
            None
        }
    }
}

impl<'id> std::iter::ExactSizeIterator for RangeIter<'id> {
    fn len(&self) -> usize {
        self.end - self.start
    }
}
