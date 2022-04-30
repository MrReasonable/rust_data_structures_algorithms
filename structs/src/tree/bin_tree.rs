use std::fmt::Debug;

#[derive(Debug)]
pub struct BinTree<T>(Option<Box<BinData<T>>>);

#[derive(Debug)]
pub struct BinData<T> {
    data: T,
    left: BinTree<T>,
    right: BinTree<T>,
}

impl<T> BinTree<T> {
    pub fn new() -> Self {
        BinTree(None)
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
            left: BinTree::default(),
            right: BinTree::default(),
        }
    }
}

impl<T: PartialOrd> BinTree<T> {
    pub fn insert(&mut self, data: T) {
        match self.0 {
            Some(ref mut bd) => {
                if data < bd.data {
                    bd.left.insert(data)
                } else {
                    bd.right.insert(data)
                }
            }
            None => self.0 = Some(Box::new(BinData::new(data))),
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
            println!("{}{:?}", spc, bd.data);
            bd.right.print_lfirst(depth + 1);
        }
    }
}

#[cfg(test)]
pub mod test {
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
        {
            let node = tree.0.as_ref().unwrap();
            assert_eq!(node.data, 5);
            assert!(node.right.0.is_none());
            assert_eq!(node.left.0.as_ref().unwrap().data, 3);
        }
        tree.insert(1);
        let node = tree.0.as_ref().unwrap();
        assert_eq!(
            node.left.0.as_ref().unwrap().left.0.as_ref().unwrap().data,
            1
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
        tree.insert(7);
        {
            let node = tree.0.as_ref().unwrap();
            assert_eq!(
                node.right
                    .0
                    .as_ref()
                    .unwrap()
                    .right
                    .0
                    .as_ref()
                    .unwrap()
                    .data,
                7
            )
        }
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
}
