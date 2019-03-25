use std::{
    fmt::{self, Debug},
    marker::PhantomData,
};

/// This struct represents an invariant lifetime `'id`.
/// The lifetime is verry important, as it builds a connection (literally)
/// between the container type, and the associated indices / ranges.
#[derive(Copy, Clone, PartialOrd, PartialEq, Eq)]
pub struct Id<'id> {
    // *mut for invariance, while still being usable in const fn.
    id: PhantomData<*mut &'id ()>,
}

impl<'id> Id<'id> {
    /// A constant constructor, only available for the crate.
    #[inline]
    pub(in crate) const fn new() -> Id<'id> {
        Self { id: PhantomData }
    }
}

/// We hold no data, but *mut prevents Send and Sync.
/// However we can safely be Send and Sync
unsafe impl<'id> Send for Id<'id> {}
unsafe impl<'id> Sync for Id<'id> {}

impl<'id> Debug for Id<'id> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Id<'id>")
    }
}
