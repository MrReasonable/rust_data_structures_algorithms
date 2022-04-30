use std::{borrow::Borrow, hash::Hash};

use super::hasher::hash;

pub(super) const BSIZE: usize = 8;

#[derive(Debug)]
pub struct BucketList<K, V> {
    seed: u64,
    len: usize,
    pub(super) buckets: Vec<Vec<(K, V)>>,
}

impl<K, V> BucketList<K, V>
where
    K: Hash + Eq,
{
    pub(super) fn new() -> Self {
        BucketList {
            seed: rand::random(),
            len: 0,
            buckets: vec![Vec::new()],
        }
    }

    pub(super) fn push(&mut self, k: K, v: V) -> usize {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        self.buckets[h].push((k, v));
        self.len += 1;
        self.buckets[h].len()
    }

    pub(super) fn get<KB>(&self, k: &KB) -> Option<&V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &self.buckets[h] {
            if k == ik.borrow() {
                return Some(iv);
            }
        }
        None
    }

    pub(super) fn get_mut<KB>(&mut self, k: &KB) -> Option<&mut V>
    where
        K: Borrow<KB>,
        KB: Hash + Eq + ?Sized,
    {
        let h = (hash(self.seed, &k) as usize) % self.buckets.len();
        for (ik, iv) in &mut self.buckets[h] {
            if k == (ik as &K).borrow() {
                return Some(iv);
            }
        }
        None
    }

    pub(super) fn bucket(&mut self, n: usize) -> Option<Vec<(K, V)>> {
        if n >= self.buckets.len() {
            return None;
        }
        let mut res = Vec::new();
        std::mem::swap(&mut res, &mut self.buckets[n]);
        self.len -= res.len();
        Some(res)
    }

    pub(super) fn set_buckets(&mut self, n: usize) {
        for _ in self.buckets.len()..n {
            self.buckets.push(Vec::new())
        }
    }
    pub(super) fn len(&self) -> usize {
        self.len
    }

    pub(super) fn b_len(&self) -> usize {
        self.buckets.len()
    }
}
