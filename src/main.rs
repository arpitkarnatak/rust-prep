
mod black_scholes;
use black_scholes::black_scholes::{BlackScholesFormulaParams, black_scholes_formula};

use crate::black_scholes::black_scholes::{black_scholes_formula_multiple_strike_prices, BlackScholesFormulaStrikePriceStreamParams, black_scholes_formula_multiple_strike_prices_async};

fn main() {
    // Black Scholes Formula for single Strike price
    println!("{:?}", black_scholes_formula({BlackScholesFormulaParams{
        S: 100.0,
        K: 100.0,
        r: 0.05,
        sigma: 0.2 ,
        T: 1.0
    }}));

    println!("{:?}", black_scholes_formula_multiple_strike_prices({BlackScholesFormulaStrikePriceStreamParams{
        S: 100.0,
        K: Vec::from([90.0,95.0,100.0,105.0,110.0]),
        r: 0.05,
        sigma: 0.2 ,
        T: 1.0
    }}));

    println!("{:?}", black_scholes_formula_multiple_strike_prices_async({BlackScholesFormulaStrikePriceStreamParams{
        S: 100.0,
        K: Vec::from([90.0,95.0,100.0,105.0,110.0]),
        r: 0.05,
        sigma: 0.2 ,
        T: 1.0
    }}));
    
}
