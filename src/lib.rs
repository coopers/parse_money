pub fn parse_money(input: &str) -> (i32, String) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let amount = parts[0].parse().unwrap();
    let currency = parts[1].to_string();
    (amount, currency)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (amount, currency) = parse_money("100 dollars");
        assert_eq!(amount, 100);
        assert_eq!(currency, "dollars");
    }
}