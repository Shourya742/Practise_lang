#![feature(rustc_private)]
extern crate rustc_driver;
fn main() {
  println!("this is a custom driver!");
  rustc_driver::main();
}