extern crate bindgen;
extern crate cmake;
use cmake::Config;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=C/compile/");
    println!("cargo:rerun-if-changed=C/database/");
    println!("cargo:rerun-if-changed=C/include/");
    println!("cargo:rerun-if-changed=C/init/");
    println!("cargo:rerun-if-changed=C/runtime/");
    println!("cargo:rerun-if-changed=C/seqio/");
    println!("cargo:rerun-if-changed=C/symbol/");
    println!("cargo:rerun-if-changed=C/util/");
    println!("cargo:rerun-if-changed=C/xcall/");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("sys/types.h")
        //.header("C/include/*")
        .header("C/include/rsm.h")
        .header("C/include/compile.h")
        .header("C/include/database.h")
        .header("C/include/error.h")
        .header("C/include/init.h")
        .header("C/include/opcode.h")
        .header("C/include/proto.h")
        .header("C/include/seqio.h")
        .header("C/include/symbol.h")
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
             .arg("lib")
             .output()
             .expect("Failed to execute command"));
    println!("cargo:rustc-link-search=native=./middle/C");
    println!("cargo:rustc-link-lib=mylib");
    println!("cargo:rustc-link-lib=framework=CoreServices");
    println!("cargo:rustc-link-lib=framework=DirectoryService");
    println!("cargo:rustc-link-lib=framework=Security");

}
