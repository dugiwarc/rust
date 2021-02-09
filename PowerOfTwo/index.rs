fn power_of_two(x: u64) -> bool {
    x.is_power_of_two()
}
fn main() {
    let a:u64 = 3;
    println!("{}, {}", power_of_two(a), (!a + 1) & a);
}