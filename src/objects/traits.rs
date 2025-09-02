use std::collections::HashSet;

pub trait ToVariants {
    fn to_variants(&self) -> Vec<u8>;
}

impl ToVariants for u8 {
    fn to_variants(&self) -> Vec<u8> {
        vec![*self]
    }
}

impl ToVariants for Vec<u8> {
    fn to_variants(&self) -> Vec<u8> {
        self.clone()
    }
}

impl ToVariants for HashSet<u8> {
    fn to_variants(&self) -> Vec<u8> {
        self.iter().copied().collect()
    }
}
