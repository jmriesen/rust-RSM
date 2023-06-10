extern crate bindgen;
extern crate cmake;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=C/init/");
    println!("cargo:rerun-if-changed=C/include/");
    println!("cargo:rerun-if-changed=C/compile/");
    println!("cargo:rerun-if-changed=C/runtime/");
    println!("cargo:rerun-if-changed=C/seqio/");
    println!("cargo:rerun-if-changed=C/util/");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("sys/types.h")
        .header("C/include/rsm.h")
        //.header("C/include/util.h")
        .header("C/include/init.h")
        .header("C/include/database.h")
        .header("C/include/proto.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build static library
    use std::process::Command;
    println!("{:?}",Command::new("make")
             .current_dir("./C")
             //.arg("Hello world")
             .output()
             .expect("Failed to execute command"));
}
