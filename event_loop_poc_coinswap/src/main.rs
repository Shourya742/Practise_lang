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
    time::Duration,
};

use epoll::{Event, Events};
use reader::MessageReader;

struct Client {
    reader: MessageReader<ClientToServerMsg>,
    address: SocketAddr,
    connected: bool,
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

        // This will wait until any of the filedescriptors reader socket fulfills.
        let event_count = epoll::wait(epoll, -1, &mut events)?;

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
                });
                continue;
            }

            for client in &mut clients {
                if fd == client.reader.stream.as_raw_fd() {
                    let msg = match client.reader.recv() {
                        Some(Ok(msg)) => msg,
                        Some(Err(error)) if error.kind() == ErrorKind::WouldBlock => {
                            continue;
                        }
                        Some(Err(error)) => {
                            eprintln!("Client {} ended with error: {error:?}", client.address);
                            client.connected = false;
                            continue;
                        }
                        None => {
                            client.connected = false;
                            continue;
                        }
                    };
        
                    eprintln!("Received msg: {msg:?}");
                }
            }


        }

        clients.retain(|c|{
            if !c.connected {
                epoll::ctl(epoll, epoll::ControlOptions::EPOLL_CTL_DEL, c.reader.stream.as_raw_fd(), Event::new(Events::EPOLLIN, c.reader.stream.as_raw_fd() as u64)).unwrap();
                c.reader.stream.shutdown(std::net::Shutdown::Both).unwrap();
            }
             c.connected});
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
