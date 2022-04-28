use std::{cell::RefCell, rc::Weak};

use crate::Rcc;

pub struct RccGraph<T, E> {
    nodes: Vec<Rcc<RccNode<T, E>>>,
}

pub struct RccNode<T, E> {
    data: T,
    edges: Vec<(E, Weak<RefCell<RccNode<T, E>>>)>,
}
