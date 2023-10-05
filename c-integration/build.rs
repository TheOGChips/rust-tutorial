fn main() {
    cc::Build::new()
        .file("src/add2.c")
        //.include("src")
        .compile("add2");
}
