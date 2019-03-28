use std::f64::consts::E; 

pub mod qfinance {
    pub struct SI {
        pub p: f64,
        pub r: f64,
        pub t: f64,
    }

    impl SI {
        pub fn simple_interest(&mut self) -> f64 {
            let si = self.p * self.r / 100. * self.t;
            return si;
        }
    }
}

pub fn simple_interest(principal: f64, rate: f64, time: f64) -> f64 {
    return principal * rate / 100. * time;
}

pub fn acc_amount(principal: f64, rate: f64, time: f64) -> f64 {
    return principal * (1.0 + rate / 100.0 * time);
}

pub fn cash_flow(income: f64, expenses: f64) -> f64 {
    return income - expenses;
}

pub fn rule_72(rate: f64) -> f64 {
    return 72. / rate;
}

pub fn compound_interest(principal: f64, rate: f64, time: f64, conversions: f64) -> f64 {
    return principal * (1.0 + rate / 100.0 / conversions).powf(time * conversions);
}

pub fn continuous_interest(principal: f64, rate: f64, time: f64) -> f64 {
    return principal * (E.powf(rate / 100.0 * time));
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_simple_interest() {
        assert_eq!(simple_interest(2000., 5., 3.), 300.);
    }
    
    #[test]
    fn test_simple_interest2() {
        let mut x = qfinance::SI { p: 2000., r: 5., t: 3. };
        assert_eq!(x.simple_interest(), 300.);
    }
    
    #[test]
    fn test_acc_amount() {
        assert_eq!(acc_amount(2000., 5.5, 3.), 2330.0);
    }
    
    #[test]
    fn test_cash_flow() {
        assert_eq!(cash_flow(2150.50, 2050.25), 100.25);
    }
    
    #[test]
    fn test_rule_72() {
        assert_eq!(rule_72(7.5), 9.6);
    }
    
    #[test]
    fn test_compound_interest() {
        assert_eq!(format!("{:.2}", compound_interest(2000., 5., 3., 12.)), "2322.94");
        assert_eq!(format!("{:.2}", compound_interest(2000., 5., 3., 365.)), "2323.64");
    }
    
    #[test]
    fn test_continuous_interest() {
        assert_eq!(format!("{:.2}", continuous_interest(2000., 5., 3.)), "2323.67");
    }
}
