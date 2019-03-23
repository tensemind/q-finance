extern crate qfinance;

use qfinance::{simple_interest, compound_interest, continuous_interest};

fn main() {
    println!("{}" , simple_interest(2000.0, 5.0, 3.0));
    println!("{}" , format!("{:.2}", compound_interest(2000., 5., 3., 365.)));
    println!("{}" , format!("{:.2}", continuous_interest(2000., 5., 3.)));
    
}
