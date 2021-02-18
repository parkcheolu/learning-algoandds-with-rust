use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::convert::TryInto;
use std::ops::Deref;

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

    pub fn pop_front(&mut self) -> Option<T> {
        match self.first.take() {
            Some(first) => {
                match Rc::try_unwrap(first) {
                    Ok(refc) => {
                        let inner = refc.into_inner();
                        self.first = inner.next;
                        if let None = self.first {
                            self.last = None;
                        };
                        Some(inner.data)
                    },
                    Err(_) => None,
                }
            },
            None => None,
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        match self.last.take() {
            Some(last) => {
                // todo: try_unwrap 에러: last의 prev의 next가 본체다.  
                match Rc::try_unwrap(Weak::upgrade(&last).unwrap()) {
                    Ok(refc) => {
                        let inner = refc.into_inner();
                        self.last = inner.prev;
                        Some(inner.data)
                    },
                    Err(_) => None,
                }
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Borrow;

    #[test]
    #[warn(unused_assignments)]
    fn test_pushs() {
        let mut dll = DbList::new();
        dll.push_front(1);
        dll.push_front(2);
        dll.push_back(3);
        dll.push_back(4);
        dll.push_front(1);

        let mut front_val = dll.pop_front().unwrap();
        front_val = dll.pop_front().unwrap();
        front_val = dll.pop_front().unwrap();
        front_val = dll.pop_front().unwrap();
        front_val = dll.pop_front().unwrap();
        println!("{:?}", front_val);
        println!("{:?}", dll);

        match Weak::upgrade(&dll.last.unwrap()) {
            Some(lastrc) => {
                println!("{:?}", lastrc);
            },
            None => {
                println!("last is none");
            }
        }
    }

    #[test]
    fn test_eq_first_last() {
        let mut dll = DbList::new();
        dll.push_back(1);
        dll.push_back(2);
        dll.push_back(3);
        dll.pop_front();
        dll.pop_front();
        if let Some(ref first) = dll.first {
            if let Some(ref last) = dll.last {
                let firstdta = (&**first).borrow().data;
                let lastdta = (&*Weak::upgrade(last).unwrap()).borrow().data;
                assert_eq!(firstdta, lastdta);
            }
        };
    }

    #[test]
    fn test_pop_front() {
        let mut dll = DbList::new();
        dll.push_back(1);
        dll.push_back(2);
        dll.pop_front();
        dll.pop_front();
        println!("after poping all data: {:?}", dll);
        dll.push_front(1);
        println!("after push_front: {:?}", dll);
    }

    #[test]
    fn test_pop_back() {
        let mut dll = DbList::new();
        dll.push_back(1);
        dll.push_back(2);
        let back = dll.pop_back().unwrap();
        assert_eq!(2, back);
        let back = dll.pop_back().unwrap();
        assert_eq!(1, back);
        println!("after poping back all data: {:?}", dll);
        dll.push_front(1);
        println!("after push_front: {:?}", dll);
    }
}