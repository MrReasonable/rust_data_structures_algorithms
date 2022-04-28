use rand::random;
use std::fmt::{self, Debug};
use std::{fmt::Write, mem::swap};

#[derive(Debug)]
struct SkipNode<T: PartialOrd> {
    right: Option<Rcc<SkipNode<T>>>,
    down: Option<Rcc<SkipNode<T>>>,
    data: Rcc<T>,
}

#[derive(Debug)]
pub struct SkipList<T: PartialOrd>(Vec<Rcc<SkipNode<T>>>);

impl<T: PartialOrd> SkipNode<T> {
    fn new(t: T) -> Self {
        SkipNode {
            right: None,
            down: None,
            data: rcc(t),
        }
    }

    pub fn insert(&mut self, data: T) -> Option<Rcc<SkipNode<T>>> {
        if let Some(ref mut rt) = self.right {
            if data > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(data);
            }
        }

        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(data) {
                Some(child) => match random::<bool>() {
                    true => {
                        let data = child.borrow().data.clone();
                        let nn = SkipNode {
                            right: self.right.take(),
                            data,
                            down: Some(child),
                        };
                        let res = rcc(nn);
                        self.right = Some(res.clone());
                        Some(res)
                    }
                    false => None,
                },
                None => None,
            };
        }

        let mut nn = SkipNode::new(data);
        nn.right = self.right.take();
        let res = rcc(nn);
        self.right = Some(res.clone());
        Some(res)
    }
}

impl<T> SkipNode<T>
where
    T: Debug + PartialOrd,
{
    pub fn print_row<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref r) = self.right {
            write!(w, ",")?;
            r.borrow().print_row(w)?;
        }
        Ok(())
    }
}

impl<T: PartialOrd> SkipList<T> {
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    pub fn insert(&mut self, data: T) {
        match self.0.len() {
            0 => {
                self.0.push(rcc(SkipNode::new(data)));
                return;
            }
            len => {
                for i in (0..len).rev() {
                    if data > *self.0[i].borrow().data.borrow() {
                        let child = self.0[i].borrow_mut().insert(data);
                        if let Some(child) = child {
                            self.loop_up(child, i + 1)
                        }
                        return;
                    }
                }
            }
        }

        let mut nn = rcc(SkipNode::new(data));
        swap(&mut nn, &mut self.0[0]);
        let res = nn;
        self.0[0].borrow_mut().right = Some(res.clone());
        self.loop_up(res, 1);
    }

    fn loop_up(&mut self, child: Rcc<SkipNode<T>>, level: usize) {
        if rand::random::<bool>() {
            return;
        }
        let data = child.borrow().data.clone();
        let mut nn = rcc(SkipNode {
            right: None,
            down: Some(child),
            data,
        });
        if level >= self.0.len() {
            self.0.push(nn);
        } else {
            swap(&mut nn, &mut self.0[level]);
            let res = nn;
            self.0[level].borrow_mut().right = Some(res.clone());
            self.loop_up(res, level + 1);
        }
    }

    pub fn exists(&self, val: &T) -> bool {
        match self.0.len() {
            0 => false,
            _ => {
                let mut node = Some(self.0[0].clone());
                while node.is_some() {
                    let sn = node.unwrap();
                    let sn = sn.borrow();
                    let data = sn.data.borrow();
                    if *data == *val {
                        return true;
                    } else if *data > *val {
                        node = match sn.right {
                            Some(ref r) if *r.borrow().data.borrow() >= *val => Some(r.clone()),
                            _ => sn.down.as_ref().cloned(),
                        };
                    } else {
                        node = sn.right.as_ref().cloned();
                    }
                }
                false
            }
        }
    }
}

impl<T: PartialOrd> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> fmt::Display for SkipList<T>
where
    T: Debug + PartialOrd,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_empty() {
            write!(f, "SkipList<Empty>")
        } else {
            for skip_node in &self.0 {
                writeln!(f)?;
                skip_node.borrow().print_row(f)?;
            }
            Ok(())
        }
    }
}

#[cfg(test)]
use crate::b_rand::rand;
use crate::{rcc, Rcc};

#[test]
fn skip_list_print() {
    let mut skip_list = SkipList::new();
    skip_list.insert(1);
    skip_list.insert(8);
    skip_list.insert(5);
    skip_list.insert(3);
    skip_list.insert(32);
    skip_list.insert(7554);
    skip_list.insert(2);
    skip_list.insert(99);
    skip_list.insert(101);
    skip_list.insert(34);
    println!("s = {}", skip_list);
}

#[test]
fn exists_returns_false_on_empty_list() {
    let t = SkipList::new();
    assert!(!t.exists(&4));
}

#[test]
fn exists_returns_true_when_value_is_in_list() {
    let mut t = SkipList::new();
    let mut values_to_find = Vec::new();
    for _ in 0..10000 {
        let v = rand(usize::MAX);
        values_to_find.push(v);
        t.insert(v);
    }
    for ref v in values_to_find {
        assert!(t.exists(v));
    }
}
