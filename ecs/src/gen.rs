#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GenData {
    pub pos: usize,
    pub gen: u64,
}

pub struct EntityActive {
    active: bool,
    gen: u64,
}

pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>,
}

impl GenManager {
    pub fn new() -> Self {
        GenManager {
            items: Vec::new(),
            drops: Vec::new(),
        }
    }

    pub fn next(&mut self) -> GenData {
        if let Some(loc) = self.drops.pop() {
            let ea = &mut self.items[loc];
            ea.active = true;
            ea.gen += 1;
            GenData {
                pos: loc,
                gen: ea.gen,
            }
        } else {
            self.items.push(EntityActive {
                active: true,
                gen: 0,
            });
            GenData {
                gen: 0,
                pos: self.items.len() - 1,
            }
        }
    }

    pub fn drop(&mut self, g: GenData) {
        if let Some(ea) = self.items.get_mut(g.pos) {
            if ea.active && ea.gen == g.gen {
                //don't drop newer items than given
                ea.active = false;
                self.drops.push(g.pos);
            }
        }
    }
}

impl Default for GenManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_items_drop() {
        let mut gm = GenManager::new();
        let g = gm.next();
        assert_eq!(g, GenData { gen: 0, pos: 0 });
        let g2 = gm.next();
        assert_eq!(g2, GenData { gen: 0, pos: 1 });
        let g3 = gm.next();
        assert_eq!(g3, GenData { gen: 0, pos: 2 });
        gm.drop(g2);
        let g4 = gm.next();
        assert_eq!(g4, GenData { gen: 1, pos: 1 });
    }
}
