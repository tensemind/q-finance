pub fn interest(principal: f64, rate: f64, _time: f64) -> f64 {
    return principal * rate / 100. * _time;
}

pub fn acc_amount(principal: f64, rate: f64, _time: f64) -> f64 {
    return principal * (1.0 + rate / 100.0 * _time);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_interest() {
        assert_eq!(interest(2000., 5., 3.), 300.);
    }
    
    #[test]
    fn test_acc_amount() {
        assert_eq!(acc_amount(2000., 5.5, 3.), 2330.0);
    }
}
