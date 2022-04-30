#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn unshift(&mut self, data: T) {
        match self.0 {
            None => self.push(data),
            Some((_, ref mut child)) => child.unshift(data),
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.0.take() {
            None => None,
            Some((x, child)) => {
                self.0 = child.0;
                Some(x)
            }
        }
    }

    pub fn shift(&mut self) -> Option<T> {
        match &mut self.0 {
            None => self.pop(),
            Some((_, child)) => match &child.0 {
                None => self.pop(),
                Some(_) => child.shift(),
            },
        }
    }
}

impl<T: PartialOrd> LinkedList<T> {
    pub fn sorted_insert(&mut self, data: T) {
        match &mut self.0 {
            None => {
                self.push(data);
            }
            Some((ref x, child)) => {
                if *x >= data {
                    self.push(data);
                } else if child.0.is_none() {
                    self.unshift(data);
                } else {
                    child.sorted_insert(data);
                }
            }
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn push_ll_adds_to_empty_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(3);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().0, 3)
    }

    #[test]
    fn push_ll_inserts_to_front_of_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(3);
        ll.push(4);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().0, 4);
    }

    #[test]
    fn unshift_ll_adds_to_empty_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.unshift(3);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().0, 3)
    }

    #[test]
    fn unshift_ll_inserts_to_back_of_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.unshift(3);
        ll.unshift(4);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().1 .0.unwrap().0, 4);
    }

    #[test]
    fn push_ll_can_be_used_with_unshift() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push(7);
        ll.unshift(3);
        ll.push(9);
        assert_eq!(
            match ll.0 {
                Some((x, _)) => x,
                _ => 0,
            },
            9
        );

        assert_eq!(ll.0.unwrap().1 .0.unwrap().1 .0.unwrap().0, 3)
    }

    #[test]
    fn sorted_insert_ll_pushes_data_in_ascending_order() {
        let mut ll = LinkedList::new();
        ll.sorted_insert(2);
        assert_eq!(ll.0.unwrap().0, 2);

        let mut ll = LinkedList::new();
        ll.sorted_insert(2);
        ll.sorted_insert(1);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, _) = child.0.unwrap();
        assert_eq!((val_1, val_2), (1, 2));

        let mut ll = LinkedList::new();
        ll.sorted_insert(1);
        ll.sorted_insert(2);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, _) = child.0.unwrap();
        assert_eq!((val_1, val_2), (1, 2));

        let mut ll = LinkedList::new();
        ll.sorted_insert(1);
        ll.sorted_insert(2);
        ll.sorted_insert(3);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, child) = child.0.unwrap();
        let (val_3, _) = child.0.unwrap();
        assert_eq!((val_1, val_2, val_3), (1, 2, 3));

        let mut ll = LinkedList::new();
        ll.sorted_insert(3);
        ll.sorted_insert(2);
        ll.sorted_insert(1);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, child) = child.0.unwrap();
        let (val_3, _) = child.0.unwrap();
        assert_eq!((val_1, val_2, val_3), (1, 2, 3));

        let mut ll = LinkedList::new();
        ll.sorted_insert(1);
        ll.sorted_insert(3);
        ll.sorted_insert(2);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, child) = child.0.unwrap();
        let (val_3, _) = child.0.unwrap();
        assert_eq!((val_1, val_2, val_3), (1, 2, 3));

        let mut ll = LinkedList::new();
        ll.sorted_insert(2);
        ll.sorted_insert(3);
        ll.sorted_insert(1);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, child) = child.0.unwrap();
        let (val_3, _) = child.0.unwrap();
        assert_eq!((val_1, val_2, val_3), (1, 2, 3));

        let mut ll = LinkedList::new();
        ll.sorted_insert(2);
        ll.sorted_insert(1);
        ll.sorted_insert(3);
        let (val_1, child) = ll.0.unwrap();
        let (val_2, child) = child.0.unwrap();
        let (val_3, _) = child.0.unwrap();
        assert_eq!((val_1, val_2, val_3), (1, 2, 3));
    }

    #[test]
    fn pop_ll_takes_from_front() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.pop(), None);
        ll.push(1);
        ll.push(2);
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
        assert_eq!(ll.pop(), None);

        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);
        assert_eq!(ll.pop(), Some(3));
        assert_eq!(ll.pop(), Some(2));
        assert_eq!(ll.pop(), Some(1));
    }

    #[test]
    fn shift_ll_takes_from_back() {
        let mut ll = LinkedList::new();
        assert_eq!(ll.shift(), None);
        ll.push(1);
        ll.push(2);
        assert_eq!(ll.shift(), Some(1));
        assert_eq!(ll.shift(), Some(2));
        assert_eq!(ll.shift(), None);

        let mut ll = LinkedList::new();
        ll.push(1);
        ll.push(2);
        ll.push(3);
        assert_eq!(ll.shift(), Some(1));
        assert_eq!(ll.shift(), Some(2));
        assert_eq!(ll.shift(), Some(3));
        assert_eq!(ll.shift(), None);
    }
}
