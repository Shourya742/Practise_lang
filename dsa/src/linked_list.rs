#![allow(dead_code)]

pub struct LinkedList<T> {
    len: usize,
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Node { value, next: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { len: 0, head: None }
    }

    pub fn push_front(&mut self, value: T) {
        if self.head.is_none() {
            self.head = Some(Box::new(Node::new(value)));
        } else {
            let mut n = Node::new(value);
            n.next = self.head.take();

            self.head = Some(Box::new(n));
        }
        self.len += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(mut h) = self.head.take() {
            self.head = h.next.take();
            self.len -= 1;
            Some(h.value)
        } else {
            None
        }
    }

    pub fn split_off(&mut self, at: usize) -> Option<Self> {
        let mut next = &mut self.head;
        let mut  i =0;
        while let Some(n) = next {
            i += 1;
            if i == at {
                let mut  l = LinkedList::new();
                l.head = n.next.take();
                l.len = self.len - 1;
                self.len -= l.len;
                return Some(l);
            }
            next = &mut n.next;
        }
        None
    }

    pub fn append(&mut self, l: Self) {
        let mut node = &mut self.head;
        while let Some(n) = node {
            node = &mut n.next;
        }
        if let Some(h) = l.head {
            node.replace(h);
            self.len += l.len;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linked_list_works() {
        let mut l = LinkedList::<usize>::new();
        assert_eq!(l.head, None);
        assert_eq!(l.len, 0);
        l.push_front(5);
        assert!(l.head.is_some());
        assert_eq!(l.len, 1);
        assert_eq!(l.pop_front(), Some(5));
        assert_eq!(l.len, 0);
        assert_eq!(l.head, None);
        l.push_front(5);
        l.push_front(4);
        l.push_front(3);
        l.push_front(2);
        assert_eq!(l.len, 4);

        let split = l.split_off(1);
        assert!(split.is_some());
        let mut split = split.unwrap();
        assert_eq!(split.pop_front(), Some(3));
        assert!(split.head.is_some());
        assert_eq!(split.len, 2);
        assert_eq!(split.pop_front(), Some(4));
        assert_eq!(split.pop_front(), Some(5));
        assert!(split.head.is_none());
        assert_eq!(split.len, 0);
        assert_eq!(l.len, 1);
        assert_eq!(l.pop_front(), Some(2));

        l.push_front(5);
        l.push_front(6);
        split.push_front(8);
        split.append(l);
        assert_eq!(split.len, 3);
        assert_eq!(split.pop_front(), Some(8));
        assert_eq!(split.pop_front(), Some(6));
        assert_eq!(split.pop_front(), Some(5));
    }
}

