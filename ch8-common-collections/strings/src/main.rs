fn main() {
    let mut s = String::new();  //NOTE: String is actually a wrapper around Vec<u8>
    let data = "initial contents";
    s = data.to_string();
    println!("data: {data}");
    s = "initial contents".to_string();
    s = String::from("initial contents");
    println!("{s}");
    s += " plus an addition";
    println!("{s}");
    s.push_str(" and a push");
    println!("{s}");
    s += " with the character ";
    s.push('x');
    println!("{s}");
    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    s = format!("{s1}-{s2}-{s3} once...");
    println!("{s}");
    s = format!("{}-{}-{} again...", s1, s2, s3);
    println!("{s}");
    
    // NOTE: The section on Strings also talked about bytes, so this seemed appropriate to test here.
    use i8 as byte;
    let x: i8 = 8;
    let y: byte = 4;
    let z: i8 = x | y;
    println!("x: {x}, y: {y}, z: {z}");
}
