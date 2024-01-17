use std::env;
use std::path::PathBuf;

fn main() {
    let libdir_path = PathBuf::from("src")
        .canonicalize()
        .expect("Cannot canonicalize path");
    let headers_path = String::from(
        libdir_path.join("struct_t.hpp")
            .to_str()
            .expect("Path is not a valid string")
    );
    let obj_path = libdir_path.join("struct_t.o");
    let lib_path = libdir_path.join("libstruct_t.a");

    println!("cargo:rustc-link-search={}", libdir_path.to_str().unwrap());
    println!("cargo:rustc-link-lib=struct_t");
    println!("cargo:rerun-if-changed={}", headers_path);

    if !std::process::Command::new("clang++")
        .arg("-c")
        .arg("-o")
        .arg(&obj_path)
        .arg(libdir_path.join("struct_t.cpp"))
        .arg("-std=c++11")  //NOTE: This is necessary to avoid some weird linking errors
        //SRC: https://users.rust-lang.org/t/undefined-c-references-during-link-step-when-statically-linking/42786
        .output()
        //.status()
        .expect("Could not spawn `clang++`")
        .status
        .success() {
            panic!("Could not compile object file");
        }

    if !std::process::Command::new("ar")
        .arg("rcs")
        .arg(lib_path)
        .arg(obj_path)
        .output()
        .expect("Could not spawn `ar`")
        .status
        .success() {
            panic!("Could not emit library file");
        }

    let bindings = bindgen::Builder::default()
        .header(headers_path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");
    //println!("{:?}", env::var("OUT_DIR"));
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("bindings.rs");
    bindings.write_to_file(out_path)
        .expect("Couldn't write bindings!");
}
