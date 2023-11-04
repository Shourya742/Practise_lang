#[cfg(test)]
mod test {

    use std::thread::{scope, sleep};
    use std::time::Duration;
    use std::{ops::AddAssign, sync::Mutex};

    #[test]
    fn test_mutex() {
        let score = Mutex::new(0u32);
        // let mut unlocked_data = score.lock().unwrap();
        // unlocked_data.add_assign(5);
        // println!("{:?}", unlocked_data);
        // drop(unlocked_data);

        let myfunc = || {
            println!("Thread 1 is waiting for mutex lock...");
            let mut data = score.lock().unwrap();
            for i in 1..10 {
                data.add_assign(i);
                println!("Thread 1 is adding {i}");
                sleep(Duration::from_millis(3000));
            }
        };
        let myfunc2 = || {
            loop {
                println!("Thread 2 is waiting for mutex lock!!");
                let guard = score.lock();

                if guard.is_ok() {
                    let mut data = guard.unwrap();
                    for i in 1..10 {
                        data.add_assign(i);
                        println!("Thread 2 is adding {i}");
                    }
                    break;
                }

                // sleep(Duration::from_millis(3));
            }
            // drop(data);
            // panic!("Error in thr ead 2");
        };
        _ = scope(|scope| {
            scope.spawn(myfunc2);
            scope.spawn(myfunc);

            // if handle1.is_err() {
            //     println!("Thread 2 had some error lets handle it here!");
            // }
        });

        println!("{:?}", score.lock().unwrap());
    }
}
