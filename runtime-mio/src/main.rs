mod executor;

fn main() {
    let (executor, spawner) = executor::new_executor_spawner();
    spawner.spawn(async_main());

    drop(spawner);

    executor.run();
}

async fn async_main() {
    let socket = executor::UdpSocket::bind("127.0.0.1:8080").unwrap();

    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf).await.unwrap();

    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, src).await.unwrap();
}
