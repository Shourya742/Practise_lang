#![allow(unreachable_code)]
#![allow(dead_code)]
#![allow(unused_variables)]
mod reader;
mod writer;
use std::{io::ErrorKind, net::{SocketAddr, TcpListener}, sync::Arc, time::Duration};

use reader::MessageReader;


struct Client {
    reader: MessageReader<ClientToServerMsg>,
    address: SocketAddr,
    connected: bool
}

fn main() -> anyhow::Result<()> {
    let stream = TcpListener::bind(("127.0.0.1", 5555))?;
    stream.set_nonblocking(true)?;

    let mut clients: Vec<Client> = Vec::new();

    loop {
        // pretty weird, we want something which gonna say wake up
        // only when some filedescriptor has some event.
        std::thread::sleep(Duration::from_millis(10));

        for client in &mut clients {
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
                None =>  {
                    client.connected = false;
                    continue;
                }
            };

            eprintln!("Received msg: {msg:?}");
        }

        clients.retain(|c| c.connected);

        let (client, address) = match stream.accept() {
            Ok(ret) =>  ret,
            Err(error) if error.kind() == ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(100));
                continue;
            }
            Err(error) => return Err(error.into())
        };

        println!("Connected from {address}");

        client.set_nonblocking(true)?;

        let client = Arc::new(client);
        clients.push(Client {
            reader: MessageReader::<ClientToServerMsg>::new(client.clone()),
            address,
            connected: true,
        });

        let mut reader = MessageReader::<ClientToServerMsg>::new(client.clone());
        reader.recv();
        
    }
    Ok(())
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum ClientToServerMsg {
    Ping
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum ServerToClientMsg {
    Pong
}
