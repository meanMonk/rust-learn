pub fn reverse(input: &str) -> String {
    // converting to chars to rev and collect back
    input.chars().rev().collect()
}
