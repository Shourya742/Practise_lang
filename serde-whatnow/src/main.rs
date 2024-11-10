// use serde::{ser::SerializeStruct, Deserialize, Serialize};
// mod serializer_json;
// mod 

// #[derive(Deserialize)]
// struct Foo {
//     a: u64,
//     b: String
// }

// impl Serialize for Foo {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//         where
//             S: serde::Serializer {
//         let mut s = serializer.serialize_struct("Foo", 2)?;
//         s.serialize_field("a", &self.a)?;
//         s.serialize_field("b", &self.b)?;
//         s.end()
//     }
// }

// fn main() {
//     println!("Hello, world!");
// }


mod de;
mod error;
mod ser;

fn main() {
    
}