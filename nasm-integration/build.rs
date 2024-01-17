/*
    NOTE: Placing the object files in the target directory also allows them to be removed when running
          "cargo clean".
*/
fn main() {
    nasm_rs::Build::new()
        .file("src/add2.asm")
        .out_dir("target/lib")
        .compile_objects()
        .unwrap();
    cc::Build::new()
        .object("target/lib/add2.o")
        .compile("add2");
}
