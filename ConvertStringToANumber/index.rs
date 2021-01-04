// s is parsed to 32-bits signed integer here (change number type if needed).
// -1 is used here as a fallback value, but any error handling instructions can be used.
// let i = match s.parse::<i32>() {
//   Ok(i) => i,
//   Err(_e) => -1,
// };


// This explicitly sets the value to 0 if it can't be parsed to an integer.
// let i: i32 = s.parse().unwrap_or(0);

fn string_to_number(s: &str) -> i32 {
    let i = match s.parse::<i32>() {
    Ok(i) => i,
    Err(_e) => -1,
    };
}
