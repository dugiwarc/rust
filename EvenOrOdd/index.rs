fn even_or_odd(i: i32) -> &'static str {
  match i % 2 {
    0 => "Even",
    _ => "Odd",
  }
}

fn main() {
    even_or_odd(4);
}
