pub use alloc::collections::*;
use alloc::vec;
use alloc::vec::Vec;
use alloc::slice::Iter;

pub struct HashMap <K, V> {
    bucket: Vec<(K, V)>,
}

impl <K, V> HashMap<K, V> {
    pub fn new() -> Self {
        Self {
            bucket: vec![]
        }
    }

    pub fn iter(&self) -> Iter<(K, V)> {
        self.bucket.iter()
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.bucket.push((key, value))
    }
}
