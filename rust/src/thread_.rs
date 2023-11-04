#[cfg(test)]
mod test {
    use std::thread::spawn;

    #[test]
    fn test_thread_basic() {
        let mut x = 0u128;

        for i in 1..500_000_000 {
            x += i;
        }
        println!("{x}");
    }
    #[test]
    fn spawn_thread() {
        let thread_fn = || {
            let mut x = 0u128;

            for i in 1..500_000_000 {
                x += i;
            }
            println!("Value of x: {x}");
        };
        println!("Starting new worker thread");
        let handle = spawn(thread_fn);
        let handle2 = spawn(thread_fn);
        println!("Worker thread completed");
        loop {
            test_thread_basic();
            if handle.is_finished() && handle2.is_finished() {
                println!("All the worker are done, let's get out of here");
                break;
            }
        }
        // test_thread_basic();
        // handle.join();
        // handle2.join();
    }
}
