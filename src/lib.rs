pub fn interest(principal: f64, rate: f64, _time: i32) -> f64 {
    return (principal * rate/100.0 * _time as f64) as f64;
}

pub fn acc_amount(principal: f64, rate: f64, _time: f64) -> f64 {
    return principal * (1.0 + rate/100.0 * _time);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interest() {
        assert_eq!(interest(2000.0, 5.0, 3), 300.0);
    }
    
    #[test]
    fn test_acc_amount() {
        assert_eq!(acc_amount(2000.0, 5.0, 3.0), 2300.0);
    }
}
