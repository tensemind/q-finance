extern crate qfinance;

use qfinance::{simple_interest, compound_interest, continuous_interest};

fn main() {
    println!("{}" , simple_interest(2000.0, 5.0, 3.0));
    println!("{}" , format!("{:.2}", compound_interest(2000., 5., 3., 365.)));
    println!("{}" , format!("{:.2}", continuous_interest(2000., 5., 3.)));
    

    let mut x = 0; // mut x: i32
    let mut done = false; // mut done: bool

    while !done {
        println!("{}" , format!("{:.2}", continuous_interest(x as f64, 5., 3.)));

        if x == 10000000 {
            done = true;
        }
        x += 1;
    }

}
