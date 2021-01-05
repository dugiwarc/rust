fn high(input: &str) -> &str {
    input.split_ascii_whitespace().rev().max_by_key(|s| s.chars().map(|c| c as u16 - 96).sum::<u16>()).unwrap_or("")
}