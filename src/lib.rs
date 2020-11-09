#[cfg(test)]
mod tests {
    use rust_decimal::Decimal;

    #[test]
    fn test_deserialization() {
        let d: Decimal = serde_json::from_str("1.1234127836128763").unwrap();
        assert_eq!(d.to_string(), "1.1234127836128763");
    }
}
