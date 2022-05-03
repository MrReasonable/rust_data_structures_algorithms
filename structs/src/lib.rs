mod b_rand;
pub mod dynamic;
mod graph;
mod hmap;
mod lists;
pub mod sorting;
mod storage;
mod tree;

type Rcc<T> = Rc<RefCell<T>>;
fn rcc<T>(t: T) -> Rcc<T> {
    Rc::new(RefCell::new(t))
}

use std::{cell::RefCell, rc::Rc};

pub use graph::Graph;
pub use hmap::hash;
pub use hmap::HMap;
pub use lists::DbList;
pub use lists::LinkedList;
pub use storage::Blob;
pub use storage::BlobError;
pub use storage::BlobStore;
pub use tree::BalancedTree;
pub use tree::BinTree;
pub use tree::HuffEncodedString;
pub use tree::SkipList;
