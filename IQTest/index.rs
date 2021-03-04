fn iq_test(numbers: &str) -> u64 {
    let rem = numbers.split_whitespace().take(3).map(|number| number.parse::<u32>().unwrap()).filter(|&i| i % 2 == 0).count() > 1

    numbers.split_whitespace().map(|number| number.parse::<u32>().unwrap()).position(|n| n % 2 == rem as u32).unwrap() as u64 + 1
}