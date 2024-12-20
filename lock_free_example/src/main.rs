use crossbeam::epoch::{self, Atomic, Owned};


struct LockFreeStack<T> {
    head: Atomic<Node<T>>
}

struct Node<T> {
    data: T,
    next: Atomic<Node<T>>
}

impl <T> LockFreeStack<T> {
    fn new() -> Self {
        Self { head: Atomic::null() }
    }

    fn push(&self, data: T) {
        let node = Owned::new(Node {
            data,
            next: Atomic::null()
        });
        let guard = epoch::pin();
        loop {
            let head = self.head.load(epoch::Ordering::Acquire,  &guard);
            node.next.store(head, epoch::Ordering::Relaxed);
            if self.head.compare_and_set(head, node, epoch::Ordering::Release, &guard).is_ok() {
                break;
            }
        }
    }

    fn pop(&self) -> Option<T> {
        let guard = epoch::pin();
        loop {
            let head = self.head.load(epoch::Ordering::Acquire, &guard);
            match unsafe {head.as_ref()} {
                Some(h) => {
                    let next = h.next.load(epoch::Ordering::Relaxed, &guard);
                    if self.head.compare_and_set(head, next, epoch::Ordering::Release, &guard).is_ok() {
                        unsafe {
                            guard.defer_destroy(head);
                        }
                        return Some(unsafe {
                            std::ptr::read(&h.data)
                        });
                    }
                },
                None => return None
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
