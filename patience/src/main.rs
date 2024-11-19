#![allow(dead_code, unused_variables)]
use std::future::Future;




fn main() {
    println!("Hello, world!");
}



async fn foo() -> usize{
    0
}


fn foo1() -> impl Future<Output = usize> {
    async {
        0
    }
}