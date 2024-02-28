use std::num::ParseIntError;

pub fn parse_money(input: &str) -> Result<(i32, String), ParseIntError> {
    let parts: Vec<&str> = input.split_whitespace().collect();
    let amount = parts[0].parse()?;
    let currency = parts[1].to_string();
    Ok((amount, currency))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_money() {
        if let Ok((amount, currency)) = parse_money("100 dollars"){
            assert_eq!(amount, 100);
            assert_eq!(currency, "dollars");
        } else {
            assert!(false);
        }
    }

    #[test]
    fn it_errors_on_floats() {
        assert!(parse_money("100.01 dollars").is_err());
    }
}