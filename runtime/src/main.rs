use std::{ future::Future, panic::catch_unwind, thread };
use std::pin::Pin;
use std::task::{ Context, Poll };
use std::time::{ Duration, Instant };
use flume::{ Sender, Receiver };

use async_task::{ Runnable, Task };
use futures_lite::future;
use http::Uri;
use hyper::{ Request, Client, Body, Response };
use once_cell::sync::Lazy;
use std::net::Shutdown;
use std::net::{ TcpStream, ToSocketAddrs };
use anyhow::{ bail, Context as _, Error, Result };
use async_native_tls::TlsStream;
use smol::{ io, prelude::*, Async };

#[derive(Debug, Clone, Copy)]
enum FutureType {
    High,
    Low,
}

trait FutureOrderLabel: Future {
    fn get_order(&self) -> FutureType;
}

fn spawn_task<F, T>(future: F, Order: FutureType) -> Task<T>
    where F: Future<Output = T> + Send + 'static, T: Send + 'static
{
    static HIGH_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> = Lazy::new(|| {
        flume::unbounded::<Runnable>()
    });

    static LOW_CHANNEL: Lazy<(Sender<Runnable>, Receiver<Runnable>)> = Lazy::new(|| {
        flume::unbounded::<Runnable>()
    });

    static HIGH_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let high_num = std::env::var("HIGH_NUM").unwrap().parse::<usize>().unwrap();
        for x in 0..high_num {
            let high_receiver = HIGH_CHANNEL.1.clone();
            let low_receiver = LOW_CHANNEL.1.clone();
            thread::spawn(move || {
                // while let Ok(runnable) = queue_one.recv() {
                //     println!("runnable accepted in queue {}", x);
                //     let _ = catch_unwind(|| runnable.run());
                // }
                match high_receiver.try_recv() {
                    Ok(runnable) => {
                        let _ = catch_unwind(|| runnable.run());
                    }
                    Err(_) => {
                        match low_receiver.try_recv() {
                            Ok(runnable) => {
                                let _ = catch_unwind(|| runnable.run());
                            }
                            Err(_) => { thread::sleep(Duration::from_millis(60)) }
                        }
                    }
                }
            });
        }

        HIGH_CHANNEL.0.clone()
    });

    static LOW_QUEUE: Lazy<flume::Sender<Runnable>> = Lazy::new(|| {
        let low_num = std::env::var("LOW_NUM").unwrap().parse::<usize>().unwrap();
        for x in 0..low_num {
            let high_receiver = HIGH_CHANNEL.1.clone();
            let low_receiver = LOW_CHANNEL.1.clone();
            thread::spawn(move || {
                loop {
                    match low_receiver.try_recv() {
                        Ok(runnable) => {
                            let _ = catch_unwind(|| runnable.run());
                        }
                        Err(_) => {
                            match high_receiver.try_recv() {
                                Ok(runnable) => {
                                    let _ = catch_unwind(|| runnable.run());
                                }
                                Err(_) => {
                                    thread::sleep(Duration::from_millis(60));
                                }
                            }
                        }
                    }
                }
            });
        }

        LOW_CHANNEL.0.clone()
    });

    let schedule_high = |runnable| HIGH_QUEUE.send(runnable).unwrap();
    let schedule_low = |runnable| LOW_QUEUE.send(runnable).unwrap();
    let schedule = match Order {
        FutureType::High => schedule_high,
        FutureType::Low => schedule_low,
    };
    let (runnable, task) = async_task::spawn(future, schedule);
    runnable.schedule();
    return task;
}

macro_rules! spawn_task {
    ($future:expr) => {
        spawn_task!($future,FutureType::Low)
    };
    ($future:expr, $order:expr) => {
        spawn_task($future,$order)
    };
}

macro_rules! join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                results.push(future::block_on($future));
            )*
            results
        }
    };
}

macro_rules! try_join {
    ($($future:expr),*) => {
        {
            let mut results = Vec::new();
            $(
                let result = catch_unwind(|| future::block_on($future));
;                results.push(result);
            )*
            results
        }
    };
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

// impl FutureOrderLabel for CounterFuture {
//     fn get_order(&self) -> FutureType {
//         self.order
//     }
// }

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

struct Runtime {
    high_num: usize,
    low_num: usize,
}

impl Runtime {
    pub fn new() -> Self {
        let num_cores = std::thread::available_parallelism().unwrap().get();
        Self {
            high_num: num_cores - 2,
            low_num: 1,
        }
    }
    pub fn with_high_num(mut self, num: usize) -> Self {
        self.high_num = num;
        self
    }

    pub fn with_low_num(mut self, num: usize) -> Self {
        self.low_num = num;
        self
    }

    pub fn run(&self) {
        std::env::set_var("HIGH_NUM", self.high_num.to_string());
        std::env::set_var("LOW_NUM", self.low_num.to_string());
        let high = spawn_task!(async {}, FutureType::High);
        let low = spawn_task!(async {}, FutureType::Low);
        join!(high, low);
    }
}

#[derive(Debug, Clone, Copy)]
struct BackgroundProcess;

impl Future for BackgroundProcess {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        println!("background provess firiing");
        std::thread::sleep(Duration::from_secs(1));
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}

struct CustomExecutor;
impl<F: Future + Send + 'static> hyper::rt::Executor<F> for CustomExecutor {
    fn execute(&self, fut: F) {
        spawn_task!(async {
            print!("Sending request");
            fut.await;
        }).detach();
    }
}

enum CustomStream {
    Plain(Async<TcpStream>),
    Tls(TlsStream<Async<TcpStream>>),
}

#[derive(Clone)]
struct CustomConnector;

impl hyper::service::Service<Uri> for CustomConnector {
    type Response = CustomStream;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;
    fn poll_ready(
        &mut self,
        cx: &mut Context<'_>
    ) -> Poll<std::prelude::v1::Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
    fn call(&mut self, uri: Uri) -> Self::Future {
        Box::pin(async move {
            let host = uri.host().context("cannot parse host")?;
            match uri.scheme_str() {
                Some("http") => {
                    let socket_addr = {
                        let host = host.to_string();
                        let port = uri.port_u16().unwrap_or(80);
                        smol
                            ::unblock(move || (host.as_str(), port).to_socket_addrs()).await?
                            .next()
                            .context("cannot resolve address")?
                    };
                    let stream = Async::<TcpStream>::connect(socket_addr).await?;
                    Ok(CustomStream::Plain(stream))
                }
                Some("https") => {
                    let socket_addr = {
                        let host = host.to_string();
                        let port = uri.port_u16().unwrap_or(443);
                        smol
                            ::unblock(move || (host.as_str(), port).to_socket_addrs()).await?
                            .next()
                            .context("cannot resolve address")?
                    };
                    let stream = Async::<TcpStream>::connect(socket_addr).await?;
                    let stream = async_native_tls::connect(host, stream).await?;
                    Ok(CustomStream::Tls(stream))
                }
                scheme => bail!("unsupported scheme: {:?}", scheme),
            }
        })
    }
}

impl tokio::io::AsyncRead for CustomStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut tokio::io::ReadBuf<'_>
    ) -> Poll<std::io::Result<()>> {
        match &mut *self {
            CustomStream::Plain(s) => {
                Pin::new(s)
                    .poll_read(cx, buf.initialize_unfilled())
                    .map_ok(|size| {
                        buf.advance(size);
                    })
            }
            CustomStream::Tls(s) => {
                Pin::new(s)
                    .poll_read(cx, buf.initialize_unfilled())
                    .map_ok(|size| { buf.advance(size) })
            }
        }
    }
}

fn main() {
    Runtime::new().with_low_num(2).with_high_num(4).run();
    // let _background = spawn_task!(BackgroundProcess).detach();
    // let one = CounterFuture { count: 0 };
    // let two = CounterFuture { count: 0 };
    // let t_one = spawn_task!(one);
    // let t_two = spawn_task!(two);
    // let t_three = spawn_task!(async {
    //     async_fn().await;
    //     async_fn().await;
    //     async_fn().await;
    //     async_fn().await;
    // });
    // std::thread::sleep(Duration::from_secs(5));
    // println!("Before the block");
    // future::block_on(t_one);
    // future::block_on(t_two);
    // future::block_on(t_three);

    let url = "http://www.rust-lang.org";
    let uri: Uri = url.parse().unwrap();
    let request = Request::builder()
        .method("GET")
        .uri(uri)
        .header("User-Agent", "hyper/0.14.2")
        .header("Accept", "text/html")
        .body(hyper::Body::empty())
        .unwrap();
    let future = async {
        let client = Client::new();
        client.request(request).await.unwrap();
    };
    let test = spawn_task!(future);
    let response = future::block_on(test);
    println!("Response status: {:?}", response);
}
