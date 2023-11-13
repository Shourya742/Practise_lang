#[cfg(test)]
mod test {
    use std::{
        cell::RefCell,
        rc::{Rc, Weak},
    };

    #[derive(Debug)]
    struct DbNode<T> {
        data: T,
        next: Option<Rc<RefCell<DbNode<T>>>>,
        prev: Option<Weak<RefCell<DbNode<T>>>>,
    }

    #[derive(Debug)]
    struct DbList<T> {
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

        fn push_front(&mut self, data: T) {
            match self.first.take() {
                Some(r) => {
                    //Create a new object
                    let new_front = Rc::new(RefCell::new(DbNode {
                        data,
                        next: Some(r.clone()),
                        prev: None,
                    }));

                    //tell the first object this is now in front of it
                    let mut m = r.borrow_mut();
                    m.prev = Some(Rc::downgrade(&new_front));
                    self.first = Some(new_front)
                }
                None => {
                    let new_data = Rc::new(RefCell::new(DbNode {
                        data,
                        next: None,
                        prev: None,
                    }));
                    self.last = Some(Rc::downgrade(&new_data));
                    self.first = Some(new_data);
                }
            }
        }
        fn push_back(&mut self, data: T) {
            match self.last.take() {
                Some(r) => {
                    //Create a new back object
                    let new_back = Rc::new(RefCell::new(DbNode {
                        data,
                        prev: Some(r.clone()),
                        next: None,
                    }));

                    //tell the last object this is now in behind it
                    let st = Weak::upgrade(&r).unwrap();
                    let mut m = st.borrow_mut();
                    self.last = Some(Rc::downgrade(&new_back));
                    m.next = Some(new_back)
                }
                None => {
                    let new_data = Rc::new(RefCell::new(DbNode {
                        data,
                        prev: None,
                        next: None,
                    }));
                    self.last = Some(Rc::downgrade(&new_data));
                    self.first = Some(new_data);
                }
            }
        }
    }

    #[test]
    fn test_doubly_linked_list() {
        let mut dl = DbList::new();
        dl.push_back(6);
        dl.push_back(11);
        dl.push_front(23);
        dl.push_front(4);
        println!("Dl = {:?}", dl);
    }
}
