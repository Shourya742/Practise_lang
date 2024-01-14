
use tokio::{net::TcpListener, io::{AsyncWriteExt, BufReader, AsyncBufReadExt}, sync::broadcast};


#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx,_rx) = broadcast::channel(10);

    loop {

        let (mut socket,addr) = listener.accept().await.unwrap();
        let tx = tx.clone();
        let mut rx = tx.subscribe();
        
        tokio::spawn(async move {

            let (reader,mut writer) =  socket.split();
    
            let mut reader = BufReader::new(reader);
            let mut line = String::new();
        
            loop {
                tokio::select! {
                    result = reader.read_line(&mut line)=>{
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(),addr)).unwrap();
                    },
                    result = rx.recv() => {
                        let (msg,other_addr) = result.unwrap(); 
                        if other_addr != addr {
                            writer.write_all(msg.as_bytes()).await.unwrap();
                            line.clear();
                        }
                    }
                }
               
            }

        });
    }
}
