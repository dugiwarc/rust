fn movie(card: i32, ticket: i32, perc: f64) -> i32 {
    let card = card as f64;
    let ticket = ticket as f64;

    let mut balance = card;
    let mut count = 0;

    while balance.ceil() >= 0.0 {
        count += 1;
        balance += ticket * perc.powi(count) - ticket;
    }

    count
}