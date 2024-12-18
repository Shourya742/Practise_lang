//! When it comes to accepting requests, we can have our main thread listening to incoming
//! TCP requests. Then our main thread distributes the requests along three different threads
//! and executors as depicted in Figure 10-1.

use std::{thread, sync::{mpsc::channel, atomic::{AtomicBool, Ordering}}, io::{self, Read, Write, ErrorKind, Cursor}, net::{TcpListener, TcpStream} };
use data_layer::data::Data;
use async_runtime::{executor::{self, Executor}, sleep::Sleep};


/// We will also like our threads to park if there are no requests to process. To communicate
/// with the threads for parking, we can ave three AtomicBools as defined with the code below.
/// 
/// Each AtomicBool represents a thread. If the AtomicBool is false, the thread is not parked.
/// If the AtomicBool is true, then our router knows that our thread is parked and that we have
/// to wake it up before sending the thread a request.
static FLAGS: [AtomicBool; 3] = [
    AtomicBool::new(false),
    AtomicBool::new(false),
    AtomicBool::new(false)
];


/// inside our threads, we create an executor, and then try to receive a message in the 
/// channel for the thread. If there is a request in the channel, we spawn a task on the
/// executor. If there is not any incoming request, we check to see if there are any task
/// waiting to be polled. If there are not any tasks, then the thread sets the FLAG to true
/// and parks the thread. If there are any tasks to be polled, then we poll the task at
/// the end of the lopp.
fn main() -> io::Result<()>{
    let (one_tx, one_rx) = channel::<TcpStream>();
    let (two_tx, two_rx) = channel::<TcpStream>();
    let (three_tx, three_rx) = channel::<TcpStream>();

    let one = thread::spawn(move || {
        let mut executor = Executor::new();
        loop {
            if let Ok(stream) = one_rx.try_recv() {
                println!("One Received connection: {}", stream.peer_addr().unwrap());
                executor.spawn(handle_client(stream));
            } else {
                if executor.polling.len() == 0 {
                    println!("One is sleeping");
                    FLAGS[0].store(true, Ordering::SeqCst);
                    thread::park();
                }
            }
            executor.poll();
        }
    });
    let two = thread::spawn(move || {
        let mut executor = Executor::new();
        loop {
            if let Ok(stream) = two_rx.try_recv() {
                println!("Two Received connection: {}", stream.peer_addr().unwrap());
                executor.spawn(handle_client(stream));
            } else {
                if executor.polling.len() == 0 {
                    println!("Two is sleeping");
                    FLAGS[1].store(true, Ordering::SeqCst);
                    thread::park();
                }
            }
            executor.poll();
        }
    });
    let three = thread::spawn(move || {
        let mut executor = Executor::new();
        loop {
            if let Ok(stream) = three_rx.try_recv() {
                println!("Three Received connection: {}", stream.peer_addr().unwrap());
                executor.spawn(handle_client(stream));
            } else {
                if executor.polling.len() == 0 {
                    println!("Three is sleeping");
                    FLAGS[2].store(true, Ordering::SeqCst);
                    thread::park();
                }
            }
            executor.poll();
        }
    });

    let router = [one_tx, two_tx, three_tx];
    let threads = [one, two, three];
    let mut index = 0;
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server listening on port 7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let _ = router[index].send(stream);
                if FLAGS[index].load(Ordering::SeqCst) {
                    FLAGS[index].store(false, Ordering::SeqCst);
                    threads[index].thread().unpark();
                }
                index+=1;
                index%=3;
            }
            Err(e) => {
                println!("Connection Failed: {}", e);
            }
        }
    }
    
    Ok(())
}


async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    stream.set_nonblocking(true)?;
    let mut buffer = Vec::new();
    let mut local_buf = [0; 1024];
    loop {
        match stream.read(&mut local_buf) {
            Ok(0) => {
                break;
            }
            Ok(len) => {
                buffer.extend_from_slice(&local_buf[..len]);
            },
            Err(ref e) if e.kind() ==  ErrorKind::WouldBlock => {
                if buffer.len() > 0 {
                    break;
                }
                Sleep::new(std::time::Duration::from_millis(10)).await;
                continue;
            },
            Err(e) => {
                println!("Failed to read from connection: {}", e);
            }
        }
    }
    match Data::deserialize(&mut Cursor::new(buffer.as_slice())) {
        Ok(message) => {
            println!("Received message: {:?}", message);
        },
        Err(e) => {
            println!("Failed to decode message: {}", e);
        }
    }
    Sleep::new(std::time::Duration::from_secs(1)).await;
    stream.write_all(b"Hello, client!")?;
    Ok(())
}