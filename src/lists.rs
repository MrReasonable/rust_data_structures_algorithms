#[derive(Debug)]
pub struct LinkedList<T>(Option<(T, Box<LinkedList<T>>)>);

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(t))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            None => self.push_front(data),
            Some((_, ref mut child)) => child.push_back(data),
        }
    }
}

impl<T: PartialOrd> LinkedList<T> {
    pub fn sorted_insert(&mut self, data: T) {
        match &mut self.0 {
            None => {
                self.push_front(data);
            }
            Some((ref x, child)) => {
                if *x >= data {
                    self.push_front(data);
                } else if child.0.is_none() {
                    self.push_back(data);
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
    fn push_front_adds_to_empty_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(3);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().0, 3)
    }

    #[test]
    fn push_front_inserts_to_front_of_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(3);
        ll.push_front(4);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().0, 4);
    }

    #[test]
    fn push_back_adds_to_empty_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().0, 3)
    }

    #[test]
    fn push_back_inserts_to_back_of_list() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_back(3);
        ll.push_back(4);
        assert!(ll.0.is_some());
        assert_eq!(ll.0.unwrap().1 .0.unwrap().0, 4);
    }

    #[test]
    fn push_front_can_be_used_with_push_back() {
        let mut ll: LinkedList<i32> = LinkedList::new();
        ll.push_front(7);
        ll.push_back(3);
        ll.push_front(9);
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
    fn sorted_insert_pushes_data_in_ascending_order() {
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
}
