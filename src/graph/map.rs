use std::{collections::HashMap, hash::Hash};

#[allow(dead_code)]
struct MapGraph<T, E, ID: Hash + Eq> {
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}
