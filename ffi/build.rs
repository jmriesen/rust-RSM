/*
 * Package: Rust Reference Standard M
 *
 * Jacob Riesen <jacobriesen@gmail.com>
 * https://github.com/jmriesen/rust-RSM
 *
 * Based on Reference Standard M by David Wicksell
 * Copyright © 2020-2024 Fourth Watch Software LC
 * https://gitlab.com/Reference-Standard-M/rsm
 *
 * Which was based on MUMPS V1 by Raymond Douglas Newman
 * Copyright © 1999-2018
 * https://gitlab.com/Reference-Standard-M/mumpsv1
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Affero General Public License (AGPL) as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Affero
 * General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see https://www.gnu.org/licenses/.
 *
 * SPDX-License-Identifier: AGPL-3.0-or-later
 */
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
    let rust_header = Path::new("../target");

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_no_includes()
        .with_language(cbindgen::Language::C)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(rust_header.join("rust.h"));

    let c_src = glob::glob("C/*/*.c")
        .unwrap()
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    cc::Build::new()
        .include("C/include")
        .include(rust_header)
        .files(&c_src)
        .flag("-Wno-deprecated")
        .flag("-fsigned-char")
        .warnings(true)
        .std("gnu99")
        .compile("cCode");

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
        .clang_arg("-funsigned-char")
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
        //NOTE the -fsigned-char flag does not seem working.
        //However after digging into they typedefs my char are signed.
        //for portability sake we might want to take another look at this but I am not going to worry about it right now.
        //.clang_arg("-fsigned-char")
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
        //.clang_arg("-fsigned-char")
        .generate()
        .expect("Unable to generate bindings");
    opcodes
        .write_to_file(out_path.join("opcodes.rs"))
        .expect("Couldn't write bindings!");

     //the C needs to link to these libraries.
    if cfg!(target_os = "linux") {
        //TODO based on the make file the crypt lib will probably be needed at some point.
        //However it builds/the tests run right now so I am delaying figuring out why this option
        //is preventing me form compiling.
        println!("cargo:rustc-link-lib=crypt");
        println!("cargo:rustc-link-lib=m");
    } else {
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=framework=DirectoryService");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}
