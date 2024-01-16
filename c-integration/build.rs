fn main() {
    cc::Build::new()
        .file("src/add2.c")
        .file("src/struct.c")
        .include("src")
        //.compile("add2");
        //.compile("struct");
        .compile("cstuff")
}
