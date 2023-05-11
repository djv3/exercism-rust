pub fn reverse(input: &str) -> String {
    let mut result = String::new();
    for char in input.chars() {
        result.insert(0, char)
    }
    result
}
