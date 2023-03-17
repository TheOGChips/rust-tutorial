fn main() {
    let s: String = String::from("Hello there");
    let word: &str = first_word(&s);
    println!("the first word is: {}", word);
}

fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
