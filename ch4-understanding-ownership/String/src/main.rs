fn main() {
    let mut s = String::from("Hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{s}");
    
    let s1: String = String::from("hello");
    let s2: String = s1.clone();
    //let s3 = s1 + " " + s2;
    println!("{s1}");
    println!("{s2}");
    //println!("{s3}");
}
