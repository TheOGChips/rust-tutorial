#[repr(C)]
pub struct Struct {
    a: i32,
    b: i16,
}

extern "C" {
    fn add2 (x: i32) -> i32;
    fn seta (s: &Struct, a: i32);
    fn setb (s: &Struct, b: i16);
    fn geta (s: &Struct) -> i32;
    fn getb (s: &Struct) -> i16;
    fn print_Struct (s: &Struct);
}

fn main() {
    let x: i32 = 5;
    let s: Struct = Struct {
        a: 0,
        b: 0,
    };
    unsafe {
        println!("add2({}): {}", x, add2(x));
        seta(&s, 4);
        setb(&s, 8);
        println!("\nPrinting using geta and getb...");
        println!("s.a: {}, s.b: {}", geta(&s), getb(&s));
        println!("\nPrinting using print_Struct...");
        print_Struct(&s);
        println!();
    }
}
