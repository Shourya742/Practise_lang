// mod foo {
//     include!(concat!(env!("OUT_DIR"), "/foo.rs"));
// }


fn main() {
    println!("{}", env!("OUT_DIR"));
    foo::foo();
}
