pub fn interest(principal: f64, rate: f64, _time: i32) -> f64 {
    return (principal * rate/100.0 * _time as f64) as f64;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interest() {
        assert_eq!(interest(2000.0, 5.0, 3), 300.0);
    }
}
