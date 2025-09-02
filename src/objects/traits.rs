use std::collections::HashSet;

pub trait Candidate {
    fn to_candidates(&self) -> Vec<u8>;
}

impl Candidate for u8 {
    fn to_candidates(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl Candidate for Vec<u8> {
    fn to_candidates(&self) -> Vec<u8> {
        self.clone()
    }
}

impl Candidate for HashSet<u8> {
    fn to_candidates(&self) -> Vec<u8> {
        self.iter().copied().collect()
    }
}
