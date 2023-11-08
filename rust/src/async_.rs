#![allow(unused_variables)]
#[cfg(test)]
mod test {

    use futures::future::FutureExt;
    use futures::{join, pin_mut, select};
    async fn get_number1() -> u8 {
        std::thread::sleep(std::time::Duration::from_millis(100));
        return 8;
    }
    async fn get_number2() -> u8 {
        std::thread::sleep(std::time::Duration::from_millis(200));
        return 1;
    }
    async fn get_number3() -> u8 {
        std::thread::sleep(std::time::Duration::from_millis(500));
        return 4;
    }
    #[test]
    fn test_async() {
        let num1 = get_number1().fuse();
        let num2 = get_number2().fuse();
        let num3 = get_number3().fuse();
        pin_mut!(num1, num2, num3);
        // let result = smol::block_on(async { join!(num1, num2, num3) });
        let result: () = smol::block_on(async {
            loop {
                select! {
                    x = num1 => println!("num1 is completed: {}",x),
                    x = num2 => println!("num2 is completed: {}",x),
                    x = num3 => println!("num3 is completed: {}",x),
                    complete => {
                        println!("All futures have finished pollling.");
                        break;
                    }
                }
            }
        });
        // println!("{:?}", result);
    }
}
