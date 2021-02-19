use std::cell::RefCell;
use std::rc::{Rc, Weak};
// 참고:
// 아래 import는 Rc<RefCell<T>>로 하여금 자신의 Borrow::borrow() 를 호출하게끔 한다. 
// 이 borrow는 self를 반환하기에 코드가 깨진다. rc.borrow() 는 auto-deref를 통해 RefCell::borrow()
// 를 호출해야 한다. 
//use std::borrow::Borrow;

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
/*
    pub fn pop_back_i(&mut self) -> Option<T> {
        let nlast: Weak<RefCell<DbNode<T>>>;
        match self.last.as_ref() {
            Some(last) => {
                let lastrc = Weak::upgrade(last).unwrap();
                match lastrc.borrow_mut().prev.as_ref() {
                    // 이전 값이 있다. last로의 모든 레퍼런스를 끊고, second-to-last를 last로
                    // 세팅하고, return data.
                    Some(lastrcp) => {
                        
                        nlast =  Rc::downgrade(&Rc::clone(&Weak::upgrade(lastrcp).unwrap()));
                        drop(last);
                        self.last = Some(nlast);
                    },
                    // 이전 값이 없다, 즉 마지막 값이니 try:unwrap하여 return data.
                    None => {

                    },
                }
                unimplemented!()
            },
            None => {
                unimplemented!()
            },
        }
    }
*/
    pub fn pop_back(&mut self) -> Option<T> {
        match self.last.take() {
            Some(last) => {
                // 기존 last의 strong reference 획득.
                let last = Weak::upgrade(&last).unwrap();
                // last와 first가 같은지 확인: 같다면 마지막 노드임을 의미한다.
                if Rc::ptr_eq(self.first.as_ref().unwrap(), &last) {
                    self.first = None;
                } else {
                    // 기존 last의 prev(the second-to-last)를 Rc로 획득.
                    let prev = Weak::upgrade(last.borrow().prev.as_ref().unwrap());
                    // 기존 second-to-last의 next(기존 last 자리)에 None을 세팅하여, old last reference 제거.
                    prev.as_ref().unwrap().borrow_mut().next = None;
                    // 기존 second-to-last를 새로운 last로 세팅.
                    self.last = Some(Rc::downgrade(prev.as_ref().unwrap()));                  
                }
                match Rc::try_unwrap(last) {
                    Ok(iv) => Some(iv.into_inner().data),
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