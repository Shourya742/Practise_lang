#[cfg(test)]
mod test {
    use std::sync::mpsc;
    use std::time::Duration;

    #[test]
    fn test_mpsc() {
        let (transmitter, receiver) = mpsc::channel::<u8>();

        let processor_code = move || {
            let mut count = 0;
            println!("Starting processor thread ....");
            loop {
                println!("Attempting to reveive message from channel...");
                let receive_result = receiver.recv_timeout(Duration::from_millis(900));
                if receive_result.is_ok() {
                    println!("Received message: {}", receive_result.unwrap());
                    count = 0;
                } else {
                    count = count + 1;
                }
                if count > 10 {
                    println!("Aborting processor thread ... no work available");
                    break;
                }
            }
        };

        for x in 1..=10 {
            let send_result = transmitter.send(x);
            println!("Send Status: {}", send_result.is_ok());
            std::thread::sleep(std::time::Duration::from_millis(400));
        }
        std::thread::spawn(processor_code).join();
    }
}
