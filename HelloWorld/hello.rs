fn greet() -> String {
    String::from("hello world!")
}

fn greet_static() -> &'static str {
    "hello world!"
}