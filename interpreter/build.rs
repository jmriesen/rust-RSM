extern crate bindgen;
use std::{
    env,
    path::{Path, PathBuf},
};

extern crate cbindgen;

#[derive(Debug)]
struct OpCodeParser;

impl bindgen::callbacks::ParseCallbacks for OpCodeParser {
    fn int_macro(&self, _name: &str, _value: i64) -> Option<bindgen::callbacks::IntKind> {
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
    let rust_header = Path::new("src/symbol_table");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_include("rsm.h")
        .exclude_item("VAR_U")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(rust_header.join("rust.h"));

    let c_src = glob::glob("src/symbol_table/*.c")
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    cc::Build::new()
        //NOTE I copied all the header files over.
        //So we are including headers for things we don't have the C code for.
        //The C code will be linked from the ffi crate.
        //I am note sure I like this as a long term solution
        //but at time of writing my only goal is to get the code to build.
        .include("src/symbol_table/*.h")
        .include(rust_header)
        .files(&c_src)
        .flag("-Wno-deprecated")
        .flag("-fsigned-char")
        .warnings(true)
        .std("gnu99")
        .compiler("/usr/bin/gcc")
        .compile("symbol_table");

    for file in c_src {
        println!(
            "cargo:rerun-if-changed={}",
            file.as_path().to_str().unwrap()
        );
    }

    println!("cargo:rerun-if-changed=src/symbol_table/*.h");
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        // note order matters so I cant just pull all .h files from that folder.
        .header("sys/types.h")
        .header("src/symbol_table/rsm.h")
        .header("src/symbol_table/compile.h")
        .header("src/symbol_table/database.h")
        .header("src/symbol_table/error.h")
        //.header("C/include/opcode.h")
        .header("src/symbol_table/proto.h")
        .header("src/symbol_table/seqio.h")
        .header("src/symbol_table/symbol.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        //NOTE the -fsigned-char flag does not seem working.
        //However after digging into they typedefs my char are signed.
        //for portability sake we might want to take another look at this but I am not going to worry about it right now.
        //.clang_arg("-fsigned-char")
        // Finish the builder and generate the bindings.
        .blocklist_type("VAR_U")
        .blocklist_type("CSTRING")
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the src/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("symbol_table_c.rs"))
        .expect("Couldn't write bindings!");

    /*
    let opcodes = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        // note order matters so I cant just pull all .h files from that folder.
        .header("C/include/opcode.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(OpCodeParser))
        //.clang_arg("-fsigned-char")
        .generate()
        .expect("Unable to generate bindings");
    opcodes
        .write_to_file(out_path.join("opcodes.rs"))
        .expect("Couldn't write bindings!");
    */

    //the C needs to link to these libraries.
    //println!("cargo:rustc-link-lib=framework=CoreServices");
    //println!("cargo:rustc-link-lib=framework=DirectoryService");
    //println!("cargo:rustc-link-lib=framework=Security");
}
