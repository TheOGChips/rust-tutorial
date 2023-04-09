#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold (&self, other: &Rectangle) -> bool {
        return self.width > other.width && self.height > other.height;
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new (value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be >= 1, got {}.", value);
        }
        else if value > 100 {
            panic!("Guess value must be <= 100, got {}.", value);
        }
        return Guess {
            value: value,
        };
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn add_two(a: i32) -> i32 {
    return a + 2;
}

pub fn greeting (name: &str) -> String {
    //return format!("Hello {}!", name);
    return format!("Hello!");
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    //#[test]
    fn another () {
        panic!("Make this test fail");
    }

    //#[test]
    fn larger_can_hold_smaller () {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(larger.can_hold(&smaller));
    }

    //#[test]
    fn smaller_cannot_hold_larger () {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        assert!(!smaller.can_hold(&larger));
    }

    //#[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    //#[test]
    fn greeting_contains_name () {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),
                "Greeting did not contain name, value was `{}`",
                result);
    }

    //#[test]
    //#[should_panic(expected = "<= 100")]    //NOTE: substr of panic msg
    fn greater_than_100 () {
        Guess::new(200);
    }

    #[test]
    fn it_works () -> Result<(), String> {
        if 2 + 2 == 4 {
            return Ok(());
        }
        else {
            return Err(String::from("2 + 2 != 4"));
        }
    }
}
