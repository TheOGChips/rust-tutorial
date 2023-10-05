extern "C" {
    fn add2 (x: i32) -> i32;
}

fn main() {
    let x: i32 = 144;
    unsafe {
        println!("\nadd2({}): {}\n", x, add2(x));
    }
}
