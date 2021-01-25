fn alphabet_position(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .filter(|c| c >= &'a' && c <= &'z')
        .map(|c| (c as u32 - 96).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}