use std::path::PathBuf;

fn main() {
    pkg_config::Config::new().atleast_version("1.0.18").probe("libsodium").unwrap();

    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default().header("wrapper.h").allowlist_function("sodium_init").allowlist_function("cryto_generichash").parse_callbacks(Box::new(bindgen::CargoCallbacks)).generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write bindings!");
}