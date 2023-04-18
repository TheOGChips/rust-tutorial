fn main() {
    assert!(r#match("foo", "foobar"));
}

fn r#match (needle: &str, haystack: &str) -> bool {
    return haystack.contains(needle);
}
