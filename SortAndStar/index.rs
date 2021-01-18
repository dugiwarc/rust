fn two_sort(arr: &[&str]) -> String {
        arr.iter()
        .min()
        .unwrap()
        .chars()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("***")
        
}

fn main() {
    let a = two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]);
    println!("{}",a)
}