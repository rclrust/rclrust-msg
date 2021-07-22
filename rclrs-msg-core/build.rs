use std::path::PathBuf;

use bindgen::Builder;

fn main() {
    let builder = Builder::default().header("src/wrapper.h");

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("bindings.rs");
    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
