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
                e.borrow_mut().prev = Some(Rc::downgrade(&new_front));
                self.first = Some(new_front);
            },
            None => {},
        }
    }
}