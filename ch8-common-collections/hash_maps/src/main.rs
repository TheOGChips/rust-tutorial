use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 25); //NOTE: Actually overwrites the original value
    scores.entry(String::from("Blue")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(50);
    
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    
    let text = "hellow world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count: &mut i8 = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
