use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::MaybeUninit,
    sync::atomic::AtomicBool,
    thread::{self, Thread},
};

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> {}

pub struct Sender<'a, T> {
    channel: &'a Channel<T>,
    receiving_thread: Thread,
}

pub struct Receiver<'a, T> {
    channel: &'a Channel<T>,
    _no_send: PhantomData<*const ()>,
}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    pub fn split<'a>(&'a mut self) -> (Sender<'a, T>, Receiver<'a, T>) {
        *self = Self::new();
        (
            Sender {
                channel: self,
                receiving_thread: thread::current(),
            },
            Receiver {
                channel: self,
                _no_send: PhantomData,
            },
        )
    }
}

impl<T> Sender<'_, T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel
            .ready
            .store(true, std::sync::atomic::Ordering::Release);
        self.receiving_thread.unpark();
    }
}

impl<T> Receiver<'_, T> {
    pub fn is_ready(&self) -> bool {
        self.channel
            .ready
            .load(std::sync::atomic::Ordering::Relaxed)
    }
    pub fn receive(self) -> T {
        while !self
            .channel
            .ready
            .swap(false, std::sync::atomic::Ordering::Acquire)
        {
            thread::park();
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

mod test {
    use std::thread;

    use super::Channel;

    #[test]
    fn test_channel() {
        let mut channel = Channel::new();
        thread::scope(|s| {
            let (sender, receiver) = channel.split();
            s.spawn(move || {
                sender.send("hello world!");
            });
            assert_eq!(receiver.receive(), "hello world!");
        })
    }
}
