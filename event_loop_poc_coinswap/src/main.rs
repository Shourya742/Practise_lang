#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
use std::{io::ErrorKind, net::TcpListener};

fn main() -> anyhow::Result<()> {
    let stream = TcpListener::bind(("127.0.0.1", 5555))?;
    stream.set_nonblocking(true)?;

    loop {
        let (client, address) = match stream.accept() {
            Ok(ret) =>  ret,
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                continue;
            }
            Err(error) => return Err(error.into())
        };

        println!("Connected from {address}");
    }
    Ok(())
}
