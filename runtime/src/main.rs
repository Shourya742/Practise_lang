use std::{ future::Future, panic::catch_unwind, thread };
use std::pin::Pin;
use std::task::{ Context, Poll };
use std::time::{ Duration, Instant };

use async_task::{ Runnable, Task };
use futures_lite::future;
use once_cell::sync::Lazy;

fn spawn_task<F, T>(future: F) -> Task<T>
    where F: Future<Output = T> + Send + 'static, T: Send + 'static
{
    static QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let (tx, rx) = flume::unbounded::<Runnable>();
        for x in 0..3 {
            let queue_one = rx.clone();
            thread::spawn(move || {
                while let Ok(runnable) = queue_one.recv() {
                    println!("runnable accepted in queue {}", x);
                    let _ = catch_unwind(|| runnable.run());
                }
            });
        }

        tx
    });

    let schedule = |runnable| QUEUE.send(runnable).unwrap();
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    println!("Here is the queue count: {:?}", QUEUE.len());
    return task;
}
struct CounterFuture {
    count: u32,
}
impl Future for CounterFuture {
    type Output = u32;
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.count += 1;
        println!("polling with result: {}", self.count);
        std::thread::sleep(Duration::from_secs(1));
        if self.count < 3 {
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(self.count)
        }
    }
}

async fn async_fn() {
    std::thread::sleep(Duration::from_secs(1));
    println!("async fn");
}

struct AsyncSleep {
    start_time: Instant,
    duration: Duration,
}
impl AsyncSleep {
    fn new(duration: Duration) -> Self {
        Self {
            start_time: Instant::now(),
            duration,
        }
    }
}

impl Future for AsyncSleep {
    type Output = bool;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let elapsed_time = self.start_time.elapsed();
        if elapsed_time >= self.duration {
            Poll::Ready(true)
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
fn main() {
    let one = CounterFuture { count: 0 };
    let two = CounterFuture { count: 0 };
    let t_one = spawn_task(one);
    let t_two = spawn_task(two);
    let t_three = spawn_task(async {
        async_fn().await;
        async_fn().await;
        async_fn().await;
        async_fn().await;
    });
    std::thread::sleep(Duration::from_secs(5));
    println!("Before the block");
    future::block_on(t_one);
    future::block_on(t_two);
    future::block_on(t_three);
}
