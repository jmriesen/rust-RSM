extern crate bindgen;
use std::{
    env,
    path::{Path, PathBuf},
};

extern crate cbindgen;

#[derive(Debug)]
struct OpCodeParser;

impl bindgen::callbacks::ParseCallbacks for OpCodeParser{
    fn int_macro(&self, _name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind>{
        Some(bindgen::callbacks::IntKind::U8)
    }
}

fn main() {
    // 1) c's header files exist.
    // 2) generate header files from rust code.
    // 3) build c object files.
    // 4) generate rust code from C headers.
    // 4) build rust code.
    // 5) link (cc handles telling cargo that it needs to link the c files.)
    let rust_header = Path::new("../target");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_no_includes()
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(&rust_header.join("rust.h"));

    let c_src = glob::glob("C/*/*.c")
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    cc::Build::new()
        .include("C/include")
        .include(rust_header)
        .files(&c_src)
        .flag("-Wno-deprecated")
        .compile("myLog");

    for file in c_src {
        println!(
            "cargo:rerun-if-changed={}",
            file.as_path().to_str().unwrap()
        );
    }

    println!("cargo:rerun-if-changed=C/include",);
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        // note order matters so I cant just pull all .h files from that folder.
        .header("sys/types.h")
        .header("C/include/rsm.h")
        .header("C/include/compile.h")
        .header("C/include/compile_temp.h")
        .header("C/include/database.h")
        .header("C/include/error.h")
        .header("C/include/init.h")
        //.header("C/include/opcode.h")
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

    let opcodes = bindgen::Builder::default()
    // The input header we would like to generate bindings for.
    // note order matters so I cant just pull all .h files from that folder.
        .header("C/include/opcode.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(OpCodeParser))
        .generate()
        .expect("Unable to generate bindings");
    opcodes
        .write_to_file(out_path.join("opcodes.rs"))
        .expect("Couldn't write bindings!");

}
