fn prints_and_returns_10 (a: i32) -> i32 {
    println!("I got the value {}", a);
    return 10;
}

pub fn add_two (a: i32) -> i32 {
    return a + 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn this_test_will_pass () {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    #[ignore]
    fn this_test_will_fail () {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }

    #[test]
    fn add_2_to_2 () {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_3_and_2 () {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred () {
        assert_eq!(102, add_two(100));
    }
}
