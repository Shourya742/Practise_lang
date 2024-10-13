use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{atomic::AtomicBool, Arc},
};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    (
        Sender { channel: a.clone() },
        Receiver { channel: a.clone() },
    )
}

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}
pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel
            .ready
            .store(true, std::sync::atomic::Ordering::Release);
    }
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel
            .ready
            .load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn receive(self) -> T {
        if !self
            .channel
            .ready
            .swap(false, std::sync::atomic::Ordering::Acquire)
        {
            panic!("Non message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

impl<T> Drop for Channel<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe {
                self.message.get_mut().assume_init_drop();
            }
        }
    }
}

mod test {
    use std::thread::{self, scope};

    use super::*;
    #[test]
    fn test_channel() {
        thread::scope(|s| {
            let (sender, receiver) = channel();
            let t = thread::current();
            s.spawn(move || {
                sender.send("hello world!");
                t.unpark();
            });
            while !receiver.is_ready() {
                thread::park();
            }
            assert_eq!(receiver.receive(), "hello world!")
        })
    }
}
