use std::{collections::VecDeque, sync::{Arc, Condvar, Mutex}};

pub struct Sender<T> {
    shared: Arc<Shared<T>>
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders +=1;
        Sender {
            shared: Arc::clone(&self.shared)
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

impl<T> Sender<T> {
    pub fn send(&mut self, t: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(t);
        drop(inner);
        self.shared.available.notify_one();
    }
}
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            return Some(t)
        }
        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(t) =>{
                    if !inner.queue.is_empty() {
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                     return Some(t);},
                None if inner.senders == 0 => {
                    return None
                },
                None => {
                    inner = self.shared.available.wait(inner).unwrap()
                }
            }
        }
    }
}
struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize
}

struct Shared<T> {
    inner: Mutex<Inner<T>>,
    available: Condvar
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: VecDeque::default(),
        senders: 1
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new()
    };
    let shared = Arc::new(shared);
    (
        Sender {
            shared: shared.clone()
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default()
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(42);
        assert_eq!(rx.recv(), Some(43));
    }
}