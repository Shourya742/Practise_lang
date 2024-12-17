//! At a high level, our executor is going to consume futures, turn them
//! into tasks so they can be run by our executor, return a handle and put the
//! task on a queue. Periodically, our executor is also going to poll tasks on 
//! that queue.

use std::{collections::VecDeque, future::{self, Future}, pin::Pin, sync::{mpsc, Arc}, task::{Context, Poll, Waker}};
use crate::waker::create_raw_waker;

/// Task struct that is going to be passed around the executor
/// 
/// Something might seem off to you when looking at the Task structc, and you would
/// be right to have this feeling. Our future in our Task struct returns a (). However
/// we will want to be able to run tasks that return data types. It would be a terrible
/// runtime if we could only return on data type. You might feel the need to pass in a 
/// generic parameter resulting in the following code:
/// 
/// pub struct Task<T> {
///     future: Pin<Box<dyn Future<Output=T> + Send>>,
///     waker: Arc<Waker>
/// }
/// 
/// However, what happens with generics is that the compiler will look at all the instances
/// of Task<T> and generate structs for every variance of T, which would just result in multiple
/// different executors for each variance of T, which would just result in a mess. Instead, we wrap
/// our future in an async block, get the result of the future, and send that result over a channel.
/// Therefore, our signature of all task return (), but we can still extract the result from the future.
pub struct Task {
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    waker: Arc<Waker>
}


pub struct Executor {
    pub polling: VecDeque<Task>
}

impl Executor {
    pub fn new() -> Self {
        Executor {
            polling: VecDeque::new()
        }
    }

    /// To initially put the task on the queue, we use the spawn function which is defined
    /// with the following code.
    /// 
    /// Here we can see how we use the channel to return a handle and convert the return value of the future to ().
    pub fn spawn<F, T>(&mut self, future: F) -> mpsc::Receiver<T> where F: Future<Output = T> + 'static + Send, T: Send + 'static {
        let (tx, rx) = mpsc::channel();
        let future: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
            let result  = future.await;
            let _ = tx.send(result);
        });

        let task = Task {
            future,
            waker: self.create_waker()
        };
        self.polling.push_back(task);
        rx
    }

    /// We have our task on our polling queue, we can poll
    /// it with the Executor's polling function
    /// 
    /// Here we can see that we just pop the task from the front of the queue
    /// wrap a reference of our Waker in a context, and pass that into the poll
    /// function of the future. If the future is ready we do not need to do anything
    /// as we are sending the result back via a channel so the future is dropped. If
    /// our future is pending we just put it back on the queue.
    pub fn poll(&mut self) {
        let mut task = match self.polling.pop_front() {
            Some(task) => task,
            None => return
        };
        let waker = task.waker.clone();
        let context = &mut Context::from_waker(&waker);
        match task.future.as_mut().poll(context) {
            Poll::Ready(()) => {},
            Poll::Pending => {
                self.polling.push_back(task);
            }
        }
    }

    pub fn create_waker(&self) -> Arc<Waker> {
        Arc::new(unsafe {
            Waker::from_raw(create_raw_waker())
        })
    }
}