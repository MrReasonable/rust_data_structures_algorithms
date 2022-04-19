use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct DbNode<T> {
    data: T,
    next: Option<Rc<RefCell<DbNode<T>>>>,
    prev: Option<Weak<RefCell<DbNode<T>>>>,
}

#[derive(Debug)]
pub struct DbList<T> {
    first: Option<Rc<RefCell<DbNode<T>>>>,
    last: Option<Weak<RefCell<DbNode<T>>>>,
}

impl<T> DbList<T> {
    pub fn new() -> Self {
        DbList {
            first: None,
            last: None,
        }
    }

    pub fn push(&mut self, data: T) {
        match self.first.take() {
            Some(r) => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: (Some(r.clone())),
                    prev: None,
                }));
                let mut m = r.borrow_mut();
                m.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
            None => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            }
        }
    }

    pub fn unshift(&mut self, data: T) {
        match self.last.take() {
            Some(r) => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    prev: (Some(r.clone())),
                    next: None,
                }));
                let st = Weak::upgrade(&r).unwrap();
                let mut m = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                m.next = Some(new_back);
            }
            None => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_back));
                self.first = Some(new_back);
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.first.take() {
            Some(first) => match Rc::try_unwrap(first) {
                Ok(refc) => {
                    let inner = refc.into_inner();
                    self.first = inner.next;
                    self.first
                        .as_ref()
                        .map(|next| next.borrow_mut().prev = None);
                    if let None = self.first {
                        self.last = None;
                    }
                    Some(inner.data)
                }
                Err(_) => None,
            },
            None => None,
        }
    }

    pub fn shift(&mut self) -> Option<T> {
        match self.last.take() {
            Some(last) => {
                let last = Weak::upgrade(&last).unwrap();
                if Rc::ptr_eq(&last, self.first.as_ref().unwrap()) {
                    self.first = None;
                } else {
                    let prev = Weak::upgrade(last.borrow().prev.as_ref().unwrap());
                    prev.as_ref().unwrap().borrow_mut().next = None;
                    self.last = Some(Rc::downgrade(prev.as_ref().unwrap()));
                }
                match Rc::try_unwrap(last) {
                    Ok(refc) => Some(refc.into_inner().data),
                    Err(_) => None,
                }
            }
            None => None,
        }
    }
}

impl<T> Default for DbList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn push_dbl_adds_to_empty_list() {
        let mut ll: DbList<i32> = DbList::new();
        ll.push(3);
        assert!(ll.first.is_some());
        assert_eq!(ll.first.as_deref().unwrap().borrow().data, 3);
        assert!(ll.last.is_some());
        assert_eq!(ll.last.unwrap().upgrade().unwrap().borrow().data, 3);
    }

    #[test]
    fn push_dbl_inserts_to_front_of_list() {
        let mut ll: DbList<i32> = DbList::new();
        ll.push(3);
        ll.push(4);
        assert!(ll.first.is_some());
        assert_eq!(ll.first.as_deref().unwrap().borrow().data, 4);
        let next = ll.first.as_deref().unwrap().borrow().next.clone();
        assert_eq!(next.as_deref().unwrap().borrow().data, 3);
        assert!(ll.last.is_some());
        assert_eq!(ll.last.unwrap().upgrade().unwrap().borrow().data, 3);
    }

    #[test]
    fn unshift_dbl_adds_to_empty_list() {
        let mut ll: DbList<i32> = DbList::new();
        ll.unshift(3);
        assert!(ll.first.is_some());
        assert_eq!(ll.first.as_deref().unwrap().borrow().data, 3);
        assert!(ll.last.is_some());
        assert_eq!(ll.last.unwrap().upgrade().unwrap().borrow().data, 3);
    }

    #[test]
    fn unshift_dbl_inserts_to_back_of_list() {
        let mut ll: DbList<i32> = DbList::new();
        ll.unshift(3);
        ll.unshift(4);
        assert!(ll.first.is_some());
        assert_eq!(ll.first.as_deref().unwrap().borrow().data, 3);
        let next = ll.first.as_deref().unwrap().borrow().next.clone();
        assert_eq!(next.as_deref().unwrap().borrow().data, 4);
        assert!(ll.last.is_some());
        assert_eq!(ll.last.unwrap().upgrade().unwrap().borrow().data, 4);
    }

    #[test]
    fn push_dbl_can_be_used_with_unshift() {
        let mut ll: DbList<i32> = DbList::new();
        //Should be 9,7,3
        ll.push(7);
        ll.unshift(3);
        ll.push(9);
        assert_eq!(ll.first.as_deref().unwrap().borrow().data, 9);
        let next = ll.first.as_deref().unwrap().borrow().next.clone();
        assert_eq!(next.as_deref().unwrap().borrow().data, 7);
        let next = next.as_deref().unwrap().borrow().next.clone();
        assert_eq!(next.as_deref().unwrap().borrow().data, 3);
        assert!(ll.last.is_some());
        assert_eq!(ll.last.unwrap().upgrade().unwrap().borrow().data, 3);
    }

    #[test]
    fn pop_dbl_takes_from_front() {
        let mut ll = DbList::new();
        assert_eq!(ll.pop(), None);
        ll.push(1);
        ll.push(2);
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);

        let mut ll = DbList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
    }

    #[test]
    fn shift_dbl_takes_from_back() {
        let mut ll = DbList::new();
        assert_eq!(ll.pop(), None);
        ll.push(1);
        ll.push(2);
        assert_eq!(ll.shift(), Some(1));
        assert_eq!(ll.shift(), Some(2));
        assert_eq!(ll.shift(), None);

        let mut ll = DbList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);
        assert_eq!(ll.shift(), Some(1));
        assert_eq!(ll.shift(), Some(2));
        assert_eq!(ll.shift(), Some(3));
    }
}
