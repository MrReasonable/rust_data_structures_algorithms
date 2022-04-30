use std::{cell::RefCell, rc::Weak};

use crate::Rcc;

#[allow(dead_code)]
type WeakNode<T, E> = Weak<RefCell<RccNode<T, E>>>;

#[allow(dead_code)]
pub struct RccGraph<T, E> {
    nodes: Vec<Rcc<RccNode<T, E>>>,
}

#[allow(dead_code)]
pub struct RccNode<T, E> {
    data: T,
    edges: Vec<(E, WeakNode<T, E>)>,
}
