use std::fmt::Debug;
#[derive(Debug, PartialEq)]
pub struct LinkedList<T>(pub(crate) Option<(T, Box<LinkedList<T>>)>);


impl<T: PartialOrd + Debug> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        // match self.0 {
        //     Some((ref mut d, ref mut n)) => {
        //         n.push_front(*d);
        //         *d = data;
        //     },
        //     None => {
        //         *self = LinkedList(Some((data, Box::new(Self::new()))));
        //     },
        // }
        let m = self.0.take();
        self.0 = Some((data, Box::new(LinkedList(m))));
    }

    pub fn push_back(&mut self, data: T) {
        match self.0 {
            Some((_, ref mut child)) => child.push_back(data),
            None => self.push_front(data),
        }
    }

    pub fn insert_sorted(&mut self, data: T) {
        if let Some((e, n)) = &mut self.0 {
            println!("e={:?}, n={:?}", e, n);
            if data > *e {
                n.insert_sorted(data);
            } else {
                self.push_front(data);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut ll = LinkedList::new();
        ll.push_front(1);
        ll.push_front(2);
        ll.push_front(3);
        ll.push_front(4);
        ll.push_front(5);
        ll.push_front(6);
        ll.push_front(7);
        ll.push_front(8);
        ll.push_front(9);
        println!("{:?}", ll);        
    }

    #[test]
    fn test_push_back() {
        let mut ll = LinkedList::new();
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        ll.push_back(4);
        ll.push_back(5);
        ll.push_back(6);
        ll.push_back(7);
        ll.push_back(8);
        ll.push_back(9);
        println!("{:?}", ll);
    }

    #[test]
    fn test_push_front_back() {
        let mut ll = LinkedList::new();
        ll.push_front(1);
        ll.push_back(2);
        ll.push_front(3);
        ll.push_back(4);
        println!("{:?}", ll);
    }

    #[test]
    fn test_insert_sort() {
        let mut ll = LinkedList::new();
        // ll.push_front(1);
        // ll.push_front(2);
        // ll.push_front(3);
        // ll.push_front(4);
        // ll.push_front(5);
        // ll.insert_sorted(4);
        ll.push_back(1);
        ll.push_back(2);
        ll.push_back(3);
        ll.push_back(5);
        ll.push_back(6);
        ll.push_back(7);
        ll.insert_sorted(4);
        println!("{:?}", ll);
    }
}