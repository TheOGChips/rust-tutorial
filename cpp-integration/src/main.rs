#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

fn main() {
    unsafe {
        let mut temp = struct_t::new();
        println!("temp.a: {}", temp.a);
        println!("temp.b: {}", temp.b);
        println!("temp.c: {}", temp.c);
        println!("temp.d: {}", temp.d);
        println!("temp.e: {}", temp.e);
        println!("temp.f: {}", temp.f);
        println!("temp.g: {}", temp.g as u8 as char);   //NOTE: A cast to u8 must be done first
        println!("temp.ret60(): {}", temp.ret60());
        println!("temp.double_this(25): {}", temp.double_this(25));

        /* ret60 must have the const declaration on the end of the function prototype in order to
         * avoid a "mutable double borrow" error thrown by the Rust compiler because of how the two
         * member functions are used here.
         */
        println!("temp.double_this(temp.ret60()): {}", temp.double_this(
            temp.ret60()
                .try_into()
                .unwrap()
            )
        );
    }
    println!("Hello, world!");
    println!("{:?}", std::env::var("OUT_DIR"));
}
