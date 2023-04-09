use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level (&self) -> i32 {
        return 3;
    }

    fn announce_and_return_part (&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        return self.part;
    }
}

fn main() {
    /*let r;
    {
        let x = 5;
        r = &x;
    }
    println!("r: {}", r);*/
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    let s: &'static str = "I have a static lifetime.";
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    }
    else {
        return y;
    }
}

fn longest_with_an_announcement<'a, T> (
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str where T: Display, {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        return x;
    }
    else {
        return y;
    }
}
