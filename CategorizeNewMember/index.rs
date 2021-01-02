const SENIOR_MIN_AGE: i32 = 55;
const SENIOR_MIN_HANDICAP: i32 = 8;

const SENIOR_CATEGORY: &'static str = "Senior";
const OPEN_CATEGORY: &'static str = "Open";

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
    data.into_iter()
        .map(|(age, handicap)| {
            if age >= SENIOR_MIN_AGE && handicap >= SENIOR_MIN_HANDICAP {
                SENIOR_CATEGORY
            } else {
                OPEN_CATEGORY
            }
        })
        .map(String::from)
        .collect::<Vec<_>>()
}