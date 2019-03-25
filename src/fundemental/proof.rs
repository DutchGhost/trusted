use std::mem;

use super::{index::Index, range::Range};

#[derive(Debug)]
pub enum Unknown {}

#[derive(Debug)]
pub enum NonEmpty {}

pub trait ProofAdd {
    type Sum;
}

impl<Q> ProofAdd for (NonEmpty, Q) {
    type Sum = NonEmpty;
}
impl<Q> ProofAdd for (Unknown, Q) {
    type Sum = Q;
}

pub trait Provable {
    type Proof;
    type WithoutProof: Provable<Proof = Unknown>;

    fn no_proof(self) -> Self::WithoutProof;
}

impl<'id, P> Provable for Index<'id, P> {
    type Proof = P;
    type WithoutProof = Index<'id, Unknown>;

    #[inline(always)]
    fn no_proof(self) -> Self::WithoutProof {
        unsafe { mem::transmute(self) }
    }
}

impl<'id, P> Provable for Range<'id, P> {
    type Proof = P;
    type WithoutProof = Range<'id, Unknown>;

    #[inline(always)]
    fn no_proof(self) -> Self::WithoutProof {
        unsafe { mem::transmute(self) }
    }
}
