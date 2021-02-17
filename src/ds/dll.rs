use std::cell::RefCell;
use std::rc::{Rc, Weak};

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

    pub fn push_front(&mut self, data: T) {
        match self.first.take() {
            Some(e) => {
                let new_front = Rc::new(RefCell::new(DbNode {
                    data,
                    next: Some(e.clone()),
                    prev: None,
                }));
                let mut me = e.borrow_mut();
                me.prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            },
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));
                self.first = Some(new_data);
            },
        }
    }

    pub fn push_back(&mut self, data: T) {
        match self.last.take() {
            Some(l) => {
                let new_back = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: Some(l.clone()),
                }));
                let st = Weak::upgrade(&l).unwrap();
                let mut ml = st.borrow_mut();
                self.last = Some(Rc::downgrade(&new_back));
                ml.next = Some(new_back);
            },
            None => {
                let new_data = Rc::new(RefCell::new(DbNode {
                    data,
                    next: None,
                    prev: None,
                }));
                self.last = Some(Rc::downgrade(&new_data));                
                self.first = Some(new_data);
            },
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.first.take() {
            Some(f) => {
                match Rc::try_unwrap(f) {
                    Ok(rc) => Some(rc.into_inner().data),
                    Err(_) => None
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut dll = DbList::new();
        dll.push_front(1);
        dll.push_front(2);
        dll.push_back(3);
        dll.push_back(4);
        dll.push_front(1);

        let front_val = dll.pop().unwrap();
        println!("{:?}", front_val);
        println!("{:?}", dll);
    }
}