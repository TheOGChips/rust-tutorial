extern "C" {
    fn add2 (x: i32) -> i32;
}

fn main() {
    let x: i32 = 5;
    unsafe {
        println!("add2({}): {}", x, add2(x));
    }
}
