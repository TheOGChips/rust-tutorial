pub fn add_two (a: i32) -> i32 {
    return internal_adder(a, 2);
}

fn internal_adder (a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
