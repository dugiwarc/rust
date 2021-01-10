fn find_short(s: &str) -> u32 {
    s.split_whitespace().map(str::len).min().unwrap()
}