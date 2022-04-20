use std::{cmp::max, fmt::Debug};

enum RotDir {
    Left,
    Right,
    None,
}

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    height: i8,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
    }

    fn height(&self) -> i8 {
        match self.0 {
            None => 0,
            Some(ref v) => v.height,
        }
    }

    fn set_height(&mut self) {
        if let Some(ref mut t) = self.0 {
            t.height = 1 + max(t.left.height(), t.right.height());
        }
    }

    fn rot_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_left());
    }

    fn rot_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rot_right());
    }
}

impl<T> Default for BinTree<T> {
    fn default() -> Self {
        BinTree::new()
    }
}

impl<T> BinData<T> {
    pub fn new(data: T) -> Self {
        BinData {
            data,
            height: 0,
            left: BinTree::default(),
            right: BinTree::default(),
        }
    }

    fn rot_left(mut self) -> Box<Self> {
        let mut res = match self.right.0.take() {
            None => return Box::new(self),
            Some(res) => res,
        };
        self.right = BinTree(res.left.0.take());
        self.right.set_height();
        res.left = BinTree(Some(Box::new(self)));
        res.left.set_height();

        res.height = 1 + max(res.left.height(), res.right.height());
        res
    }

    fn rot_right(mut self) -> Box<Self> {
        let mut res = match self.left.0.take() {
            None => return Box::new(self),
            Some(res) => res,
        };
        self.left = BinTree(res.right.0.take());
        self.left.set_height();
        res.right = BinTree(Some(Box::new(self)));
        res.right.set_height();

        res.height = 1 + max(res.left.height(), res.right.height());
        res
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn insert(&mut self, data: T) {
        let rot_dir = match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.insert(data);
                    if bd.left.height() - bd.right.height() > 1 {
                        RotDir::Right
                    } else {
                        RotDir::None
                    }
                } else {
                    bd.right.insert(data);
                    if bd.right.height() - bd.left.height() > 1 {
                        RotDir::Left
                    } else {
                        RotDir::None
                    }
                }
            }
            None => {
                self.0 = Some(Box::new(BinData::new(data)));
                RotDir::None
            }
        };

        match rot_dir {
            RotDir::Left => self.rot_left(),
            RotDir::Right => self.rot_right(),
            RotDir::None => self.set_height(),
        }
    }

    pub fn exists(&self, data: T) -> bool {
        match self.0 {
            Some(ref bd) => {
                if data == bd.data {
                    true
                } else if data < bd.data {
                    bd.left.exists(data)
                } else {
                    bd.right.exists(data)
                }
            }
            None => false,
        }
    }
}

impl<T> BinTree<T>
where
    T: Debug,
{
    pub fn print_lfirst(&self, depth: usize) {
        if let Some(ref bd) = self.0 {
            bd.left.print_lfirst(depth + 1);
            let mut spc = String::with_capacity(depth);
            for _ in 0..depth {
                spc.push('.');
            }
            println!("{}:{}{:?}", bd.height, spc, bd.data);
            bd.right.print_lfirst(depth + 1);
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::b_rand::rand;

    use super::*;

    #[test]
    fn insert_adds_first_item_to_root() {
        let mut tree = BinTree::new();
        tree.insert(1);
        assert_eq!(tree.0.as_ref().unwrap().data, 1);
    }

    #[test]
    fn insert_adds_lower_numbers_to_left() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(3);
        let node = tree.0.as_ref().unwrap();
        assert_eq!(node.data, 5);
        assert!(node.right.0.is_none());
        assert_eq!(node.left.0.as_ref().unwrap().data, 3);
    }

    #[test]
    fn imbalance_left_is_corrected_with_right_rotation() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(4);
        let node = tree.0.as_ref().unwrap();
        assert_eq!(node.data, 3);
        assert!(node.left.0.is_none());
        assert_eq!(node.right.0.as_ref().unwrap().data, 5);
        assert_eq!(
            node.right.0.as_ref().unwrap().left.0.as_ref().unwrap().data,
            4
        );
    }

    #[test]
    fn insert_adds_equal_numbers_to_right() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(5);
        let node = tree.0.as_ref().unwrap();
        assert_eq!(node.data, 5);
        assert!(node.left.0.is_none());
        assert_eq!(node.right.0.as_ref().unwrap().data, 5)
    }

    #[test]
    fn insert_adds_higher_numbers_to_right() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(6);
        {
            let node = tree.0.as_ref().unwrap();
            assert_eq!(node.data, 5);
            assert!(node.left.0.is_none());
            assert_eq!(node.right.0.as_ref().unwrap().data, 6)
        }
    }

    #[test]
    fn imbalance_right_is_corrected_with_left_rotation() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(7);
        tree.insert(6);
        let node = tree.0.as_ref().unwrap();
        assert_eq!(node.data, 7);
        assert!(node.right.0.is_none());
        assert_eq!(node.left.0.as_ref().unwrap().data, 5);
        assert_eq!(
            node.left.0.as_ref().unwrap().right.0.as_ref().unwrap().data,
            6
        );
    }

    #[test]
    pub fn exists_returns_false_when_tree_is_empty() {
        let tree = BinTree::new();
        assert_eq!(tree.exists(4), false)
    }

    #[test]
    pub fn exists_returns_false_when_value_not_in_tree() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(6);
        tree.insert(3);
        tree.insert(7);
        tree.insert(9);
        tree.insert(17);
        assert_eq!(tree.exists(4), false)
    }

    #[test]
    pub fn exists_returns_true_when_value_not_in_tree() {
        let mut tree = BinTree::new();
        tree.insert(5);
        tree.insert(2);
        tree.insert(6);
        tree.insert(3);
        tree.insert(7);
        tree.insert(9);
        tree.insert(17);
        assert_eq!(tree.exists(5), true);
        assert_eq!(tree.exists(2), true);
        assert_eq!(tree.exists(6), true);
        assert_eq!(tree.exists(3), true);
        assert_eq!(tree.exists(7), true);
        assert_eq!(tree.exists(9), true);
        assert_eq!(tree.exists(17), true);
    }

    #[test]
    fn tree_height_is_set_to_num_levels_plus_one() {
        let mut tree = BinTree::new();
        tree.insert(10);
        assert_eq!(tree.0.as_ref().unwrap().height, 1);
        tree.insert(15);
        assert_eq!(tree.0.as_ref().unwrap().height, 2);
        tree.insert(7);
        assert_eq!(tree.0.as_ref().unwrap().height, 2);
        tree.insert(16);
        assert_eq!(tree.0.as_ref().unwrap().height, 3);
        tree.insert(14);
        assert_eq!(tree.0.as_ref().unwrap().height, 3);
        tree.insert(8);
        assert_eq!(tree.0.as_ref().unwrap().height, 3);
        tree.insert(6);
        assert_eq!(tree.0.as_ref().unwrap().height, 3);
        tree.insert(5);
        assert_eq!(tree.0.as_ref().unwrap().height, 4);
        tree.insert(4);
        assert_eq!(tree.0.as_ref().unwrap().height, 4);
    }

    #[test]
    fn tree_balances_with_huge_unordered_set() {
        let mut tree = BinTree::new();
        for _ in 0..100000 {
            let num = rand(100000000);
            tree.insert(num);
        }
        assert!(tree.0.as_ref().unwrap().height < 22);
    }

    #[test]
    fn tree_balances_with_huge_ordered_set() {
        let mut tree = BinTree::new();
        for i in 0..100000 {
            tree.insert(i);
        }
        assert_eq!(tree.0.as_ref().unwrap().height, 17);

        let mut tree = BinTree::new();
        for i in (0..100000).rev() {
            tree.insert(i);
        }
        assert_eq!(tree.0.as_ref().unwrap().height, 17);
    }
}
