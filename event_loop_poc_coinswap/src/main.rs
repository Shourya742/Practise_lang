#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod reader;
mod writer;
use std::{
    io::ErrorKind,
    net::{SocketAddr, TcpListener},
    os::fd::{AsRawFd, RawFd},
    sync::Arc,
    time::{Duration, Instant},
};

use epoll::{Event, Events};
use reader::MessageReader;

struct Client {
    reader: MessageReader<ClientToServerMsg>,
    address: SocketAddr,
    connected: bool,
    last_activity: Instant
}

const TIMEOUT_DURATION: Duration =  Duration::from_secs(4);

impl Client {
    fn until_timeout(&self) -> Duration {
        let since_last_activity = self.last_activity.elapsed();
        TIMEOUT_DURATION.saturating_sub(since_last_activity)
    }
}

fn main() -> anyhow::Result<()> {
    let server = TcpListener::bind(("127.0.0.1", 5555))?;
    server.set_nonblocking(true)?;

    let mut clients: Vec<Client> = Vec::new();

    let epoll = epoll::create(false)?;

    epoll::ctl(
        epoll,
        epoll::ControlOptions::EPOLL_CTL_ADD,
        server.as_raw_fd(),
        Event::new(Events::EPOLLIN, server.as_raw_fd() as u64),
    )?;

    loop {
        // pretty weird, we want something which gonna say wake up
        // only when some filedescriptor has some event.
        // std::thread::sleep(Duration::from_millis(10));


        let mut events = [Event::new(Events::empty(), 0); 1024];

        // Max time till we make the epoll blocking based on disconnection from the socket.
        // -1 for infinite, like till any of the fd gets ready
        // t minimum disconnection time.
        let time_until_next_timeout = clients.iter().map(|c| c.until_timeout()).map(|d| d.as_millis() as i32).min().unwrap_or(-1);

        eprintln!("Time until timeout: {time_until_next_timeout}");

        // This will wait until any of the filedescriptors reader socket fulfills.
        // For coinswap we have a shutdown schemeing, so makes sense to have a timeout to epoll
        // rather then being blocking.
        // default epoll is level triggered, should we do edge trigger.. not sure..
        let event_count = epoll::wait(epoll, time_until_next_timeout, &mut events)?;

        for event in &events[..event_count] {
            let fd = event.data as RawFd;

            if fd == server.as_raw_fd() {
                let (client, address) = match server.accept() {
                    Ok(ret) => ret,
                    Err(error) if error.kind() == ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(100));
                        continue;
                    }
                    Err(error) => return Err(error.into()),
                };
        
                println!("Connected from {address}");
        
                client.set_nonblocking(true)?;
        
                // Registering in case there is any change in input of any of the socket.
                epoll::ctl(
                    epoll,
                    epoll::ControlOptions::EPOLL_CTL_ADD,
                    server.as_raw_fd(),
                    Event::new(Events::EPOLLIN, server.as_raw_fd() as u64),
                )?;
        
                let client = Arc::new(client);
                clients.push(Client {
                    reader: MessageReader::<ClientToServerMsg>::new(client.clone()),
                    address,
                    connected: true,
                    last_activity: Instant::now()
                });
                continue;
            }

            for client in &mut clients {
                if fd == client.reader.stream.as_raw_fd() {
                    handle_client(client).unwrap();   
                }
            }
        }

        for client in &mut clients {
            if client.until_timeout() == Duration::ZERO {
                client.connected = false;
            }
        }

        clients.retain(|c|{
            if !c.connected {
                epoll::ctl(epoll, epoll::ControlOptions::EPOLL_CTL_DEL, c.reader.stream.as_raw_fd(), Event::new(Events::EPOLLIN, c.reader.stream.as_raw_fd() as u64)).unwrap();
                c.reader.stream.shutdown(std::net::Shutdown::Both).unwrap();
                eprintln!("Client disconnected: {}", c.address);
            }
            c.connected

            });
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum ClientToServerMsg {
    Ping,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum ServerToClientMsg {
    Pong,
}


fn handle_client(client: &mut Client) -> anyhow::Result<()> {
    let msg = match client.reader.recv() {
        Some(Ok(msg)) => msg,
        Some(Err(error)) if error.kind() == ErrorKind::WouldBlock => {
            return Ok(());
        }
        Some(Err(error)) => {
            eprintln!("Client {} ended with error: {error:?}", client.address);
            client.connected = false;
            return Ok(());
        }
        None => {
            client.connected = false;
            return Ok(());
        }
    };

    client.last_activity = Instant::now();

    eprintln!("Received msg: {msg:?}");   

    Ok(())
}