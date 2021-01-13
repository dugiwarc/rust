fn longest_consec(strarr: Vec<&str>, k:usize) -> String {
    if k > strarr.len() || k == 0 || strarr.len() == 0 { String::new() } else {
        strarr.windows(k).map(|x| {x.join("")}).rev().max_by_key(String::len).unwrap()
    }
}

fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut result = String::new();

    if k > 0 && strarr.len() >= k {
        for index in 0..strarr.len() - k + 1 {
            let s: String = strarr[index..index + k].join("");
            if s.len() > result.len() {
                result = s;
            }
        }
    }

    result
}