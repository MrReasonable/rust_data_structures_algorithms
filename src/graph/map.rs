use std::{collections::HashMap, hash::Hash};

struct MapGraph<T, E, ID: Hash + Eq> {
    mp: HashMap<ID, T>,
    edges: Vec<(E, ID, ID)>,
}
