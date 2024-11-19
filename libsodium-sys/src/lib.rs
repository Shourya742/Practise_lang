#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}


pub use ffi::sodium_init;


#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct Sodium:

impl Sodium {
    pub fn new() -> Result<Self, ()> {
        if unsafe {
            ffi::sodium_init()
        } < 0 {
            Err(())
        } else {
            Ok(())
        }
    }

    pub fn cryto_generichash(&self) -> () {}
}