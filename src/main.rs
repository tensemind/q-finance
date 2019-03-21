extern crate qfinance;

use qfinance::interest;

fn main() {
    println!("{}" , interest(2000.0, 5.0, 3));
}
