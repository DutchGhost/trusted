use super::{id::Id, proof::NonEmpty};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// An id based index, trough which a container
/// can be accessed without boundschecks.
#[derive(Debug)]
pub struct Index<'id, Proof = NonEmpty> {
    pub(crate) index: usize,
    id: Id<'id>,
    proof: PhantomData<Proof>,
}

impl<'id, P> Index<'id, P> {
    /// Creates a new Index from `index`.
    /// This function is marked unsafe,
    /// because `index` could come from anywhere,
    /// and is therefore not known to be valid.
    #[inline(always)]
    pub const unsafe fn new(index: usize) -> Index<'id, P> {
        Index {
            id: Id::new(),
            index,
            proof: PhantomData,
        }
    }

    /// Return the index as an integer offset from the start of the container.
    #[inline(always)]
    pub const fn integer(&self) -> usize {
        self.index
    }
}

impl<'id, P> Copy for Index<'id, P> {}

impl<'id, P> Clone for Index<'id, P> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

/// Index can only be compared with other indices of the same branding
impl<'id, P, Q> PartialEq<Index<'id, Q>> for Index<'id, P> {
    #[inline(always)]
    fn eq(&self, rhs: &Index<'id, Q>) -> bool {
        self.index == rhs.index
    }
}

impl<'id, P> Eq for Index<'id, P> {}

impl<'id, P, Q> PartialOrd<Index<'id, Q>> for Index<'id, P> {
    #[inline(always)]
    fn partial_cmp(&self, rhs: &Index<'id, Q>) -> Option<Ordering> {
        Some(self.index.cmp(&rhs.index))
    }

    #[inline(always)]
    fn lt(&self, rhs: &Index<'id, Q>) -> bool {
        self.index < rhs.index
    }
}

impl<'id, P> Ord for Index<'id, P> {
    #[inline(always)]
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.index.cmp(&rhs.index)
    }
}

impl<'id, P> Hash for Index<'id, P> {
    #[inline(always)]
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.index.hash(h)
    }
}
