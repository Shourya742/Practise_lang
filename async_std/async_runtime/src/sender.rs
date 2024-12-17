//! When it comes to sending data over a TCP socket, we must allow our 
//! executor to switch over to another async task if the connection
//! is currently blocked. If the connection is not blocked, then we can
//! write the bytes to the stream.

use std::{
    future::Future,
    task::{Context, Poll},
    pin::Pin,
    net::TcpStream,
    io::{self, Write},
    sync::{Arc, Mutex}
};

/// Our sender is essentially a future. During the poll function, if the
/// stream is blocking we will return a pending, if the stream is not 
/// blocking, we write the bytes to the stream.
/// 
/// Our TcpStream is wrapped in an Arc<Mutex<T>>. We use the Arc<Mutex<T>> so
/// we can pass the TcpStream into the Sender and Receiver. Once we have sent
/// bytes over the stream, we will want to employ a Receiver future to await for
/// the response.
pub struct TcpSender {
    pub stream: Arc<Mutex<TcpStream>>,
    pub buffer: Vec<u8>
}

impl Future for TcpSender {
    type Output = io::Result<()>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut stream = match self.stream.try_lock() {
            Ok(stream) => stream,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };
        stream.set_nonblocking(true)?;
        match stream.write_all(&self.buffer) {
            Ok(_) => {
                Poll::Ready(Ok(()))
            },
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                cx.waker().wake_by_ref();
                Poll::Pending
            },
            Err(e) => Poll::Ready(Err(e))
        }
    }
}