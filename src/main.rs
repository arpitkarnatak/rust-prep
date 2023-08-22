
mod black_scholes;
use black_scholes::black_scholes::{BlackScholesFormulaParams, black_scholes_formula};

fn main() {
    // Black Scholes Formula for single Strike price
    println!("{:?}", black_scholes_formula({BlackScholesFormulaParams{
        S: 100.0,
        K: 100.0,
        r: 0.05,
        sigma: 0.2 ,
        T: 1.0
    }}));
}
