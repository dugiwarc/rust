// extern crate itertools;
// use itertools::Itertools;

// fn descending_order_og(x: u64) -> u64 {
//     x.to_string()
//         .chars()
//         .sorted()
//         .rev()
//         .collect::<String>()
//         .parse::<u64>()
//         .unwrap()
// }

fn descending_order(x: u64) -> u64 {
    let mut chars: Vec<char> = x.to_string().chars().collect();
    chars.sort_by(|a, b| b.cmp(a));
    chars.into_iter().collect::<String>().parse().unwrap()
}

fn main(){
    let a = descending_order(5235);
    println!("{}",a)
}