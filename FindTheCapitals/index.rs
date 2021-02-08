fn capital(capitals: &[Capital]) -> Vec<String> {
    capitals.iter().map(|capital|
        format!("The capital of {} is {}",
        capital.country.or(capital.state).unwrap(),
        capital.capital
    )
    ).collect()
}