fn even_or_odd(i: i32) -> &'static str {
    if i % 2 == 0 { 
        println!("Even");
        return "Even" 
    }
    else {
        println!("Odd");
        return "Odd"
    } 
}

fn main() {
    even_or_odd(4);
}