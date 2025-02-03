#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod reader;
mod writer;
use std::{io::ErrorKind, net::TcpListener, time::Duration};

fn main() -> anyhow::Result<()> {
    let stream = TcpListener::bind(("127.0.0.1", 5555))?;
    stream.set_nonblocking(true)?;

    loop {
        let (client, address) = match stream.accept() {
            Ok(ret) =>  ret,
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(error) => return Err(error.into())
        };

        println!("Connected from {address}");
    }
    Ok(())
}


enum ClientToServerMsg {
    Ping
}

enum ServerToClientMsg {
    Pong
}