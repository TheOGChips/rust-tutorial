fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    //println!("The length of '{}' is {}.", s1, len);
    println!("The length of '{s1}' is {len}");
    
    let mut s: String = String::from("hello");
    println!("s: {s}");
    change(&mut s);
    println!("s: {s}");
}

fn calculate_length (s: &String) -> usize {
    return s.len();
}

fn change (str: &mut String) {
    str.push_str(" there");
}
