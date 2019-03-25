use super::{
    id::Id,
    index::Index,
    proof::{NonEmpty, ProofAdd, Unknown},
};

use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// A range is a range into a container.
/// The range has an extra `Proof` parameter,
/// which indicates whether the range is know to be non-empty (NonEmpty),
/// or not (Unknown).
/// A NonEmpty range has a length of 1 or more.
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

impl<'id, P, Q> PartialEq<Range<'id, Q>> for Range<'id, P> {
    fn eq(&self, other: &Range<'id, Q>) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl<'id, P> Eq for Range<'id, P> {}

impl<'id, P> Hash for Range<'id, P> {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.start.hash(h);
        self.end.hash(h);
    }
}

impl<'id, P> Range<'id, P> {
    /// Attemts to create a NonEmpty range, returning Some on success, None on failure.
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

    /// Split the range at `index`. if past the end, return false and clamp to the end.
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

    /// Returns Some if `index` is contained within the range.
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

    /// Join together two adjacent ranges (they must be exactly touching, and
    /// in left to right order).
    #[inline]
    pub const fn join<Q>(
        &self,
        other: Range<'id, Q>,
    ) -> Option<Range<'id, <(P, Q) as ProofAdd>::Sum>>
    where
        (P, Q): ProofAdd,
    {
        unsafe {
            [Some(Range::from_any(self.start, other.end)), None][(self.end != other.start) as usize]
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
    /// Creates an unchecked NonEmpty range.
    /// # Unsafe
    /// This function is marked unsafe,
    /// because it's not checked whether the range is truely NonEmpty.
    #[inline]
    pub const unsafe fn nonempty_unchecked(&self) -> Range<'id, NonEmpty> {
        Range::from_any(self.start, self.end)
    }

    /// Returns the first Index of the range.
    #[inline]
    pub const fn first(&self) -> Index<'id, P> {
        unsafe { Index::new(self.start) }
    }

    /// Returns the middle Index of the range.
    #[inline]
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
    /// Splits the range at `index`.
    /// # Unsafe
    /// This function is marked unsafe,
    /// because `index` could be the first, or last index of the range,
    /// therefore breaking the NonEmpty variant.
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

    /// Returns the last Index of the range.
    #[inline]
    pub const fn last(&self) -> Index<'id> {
        unsafe { Index::new(self.end - 1) }
    }

    /// Returns a new range,
    /// containing indices from *this* range's second index to *this* range's end index.
    #[inline]
    pub const fn tail(self) -> Range<'id> {
        unsafe { Range::from(self.start + 1, self.end) }
    }

    /// Returns a new range,
    /// containing indices from *this* range's first index, to *this* range's second last index.
    #[inline]
    pub const fn head(self) -> Range<'id> {
        unsafe { Range::from(self.start, self.end - 1) }
    }

    /// Advance's the range backwards.
    /// Returns true if start < end after advancing.
    #[inline]
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

    /// Advance's the range forwards.
    /// Returns true if start < end after advancing.
    #[inline]
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

/// An Iterator between the range `start..end`.
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
