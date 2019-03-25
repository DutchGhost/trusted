use std::mem;

use super::{index::Index, range::Range};

#[derive(Debug)]
/// Represents an unknown length. This might be 0.
pub enum Unknown {}

#[derive(Debug)]
/// Represents aa length known to be non-zero.
pub enum NonEmpty {}

/// A trait representing the sum of proof P and Q.
pub trait ProofAdd {
    type Sum;
}

/// NonEmpty + Q = NonEmpty
impl<Q> ProofAdd for (NonEmpty, Q) {
    type Sum = NonEmpty;
}

/// Unknown + Q = Q.
impl<Q> ProofAdd for (Unknown, Q) {
    type Sum = Q;
}

/// Represents the proofs a type can have, and how to discard the proof.
pub trait Provable {
    type Proof;
    type WithoutProof: Provable<Proof = Unknown>;

    /// Return a copy of `self` with the proof parameter set to `Unknown`.
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
