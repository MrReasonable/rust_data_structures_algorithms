use self::bucket_list::{BucketList, BSIZE};
use std::{borrow::Borrow, hash::Hash};

mod bucket_list;
mod hasher;

#[derive(Debug)]
pub struct HMap<K, V> {
    n_moved: usize,
    main: BucketList<K, V>,
    grow: BucketList<K, V>,
}

impl<K, V> HMap<K, V>
where
    K: Hash + Eq,
{
    pub fn new() -> Self {
        HMap {
            n_moved: 0,
            main: BucketList::new(),
            grow: BucketList::new(),
        }
    }

    pub fn insert(&mut self, k: K, v: V) {
        if let Some(iv) = self.main.get_mut(&k) {
            *iv = v;
        } else if let Some(iv) = self.grow.get_mut(&k) {
            *iv = v;
        } else if self.n_moved > 0 {
            self.grow.push(k, v);
            self.move_bucket();
        } else if self.main.push(k, v) > BSIZE / 2 {
            self.move_bucket();
        }
    }

    pub fn get_mut<KR>(&mut self, kr: &KR) -> Option<&mut V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        if let Some(b) = self.main.get_mut(kr) {
            Some(b)
        } else {
            self.grow.get_mut(kr)
        }
    }

    pub fn get<KR>(&mut self, kr: &KR) -> Option<&V>
    where
        K: Borrow<KR>,
        KR: Hash + Eq + ?Sized,
    {
        self.main.get(kr).or_else(|| self.grow.get(kr))
    }

    pub fn len(&self) -> usize {
        self.main.len() + self.grow.len()
    }

    pub fn is_empty(&self) -> bool {
        self.main.len() == 0 && self.grow.len() == 0
    }

    fn move_bucket(&mut self) {
        if self.n_moved == 0 {
            self.grow.set_buckets(self.main.b_len() * 2)
        }

        if let Some(b) = self.main.bucket(self.n_moved) {
            for (k, v) in b {
                self.grow.push(k, v);
            }
            self.n_moved += 1;
        } else {
            std::mem::swap(&mut self.main, &mut self.grow);
            self.n_moved = 0;
        }
    }
}

impl<K, V> Default for HMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_right_values() {
        let mut hm = HMap::new();
        hm.insert("james", 18);
        hm.insert("dave", 45);
        hm.insert("andy", 23);
        hm.insert("pete", 14);
        hm.insert("steve", 90);
        hm.insert("jane", 105);
        hm.insert("grader", 23);
        hm.insert("irene", 65);
        hm.insert("sam", 66);
        hm.insert("andrex", 77);
        hm.insert("andrew", 89);
        hm.insert("geralt", 99);
        //Change dave's age
        hm.insert("dave", 70);

        assert_eq!(hm.get("geralt"), Some(&99));
        assert_eq!(hm.get("steve"), Some(&90));
        assert_eq!(hm.get("grader"), Some(&23));
        assert_eq!(hm.get("dave"), Some(&70));
        assert_eq!(hm.len(), 12);
    }

    #[test]
    fn test_lots_of_numbers() {
        let mut hm = HMap::new();
        for x in 0..10000 {
            hm.insert(x, x + 250);
        }

        assert_eq!(hm.len(), 10000);
        assert_eq!(hm.get(&6000), Some(&6250));

        for (n, x) in hm.main.buckets.iter().enumerate() {
            assert!(x.len() < 10, "main bucket too big {}:{}", n, x.len());
        }

        for (n, x) in hm.grow.buckets.iter().enumerate() {
            assert!(x.len() < 10, "grow bucket too big {}:{}", n, x.len());
        }
    }
}
