extern crate gdnative_bindings_generator;

use gdnative_bindings_generator::*;
use std::path::PathBuf;
use std::env;
use std::fs::File;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut output = File::create(out_path.join("editor_types.rs")).unwrap();

    generate_bindings(&mut output, Crate::editor).unwrap();
}
