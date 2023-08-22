pub mod black_scholes {
    use tokio::task;

    #[derive(Debug)]
    pub struct BlackScholesFormulaParams {
        pub S: f64,
        pub K: f64,
        pub r: f64,
        pub sigma: f64,
        pub T: f64
    }

    pub struct BlackScholesFormulaStrikePriceStreamParams {
        pub S: f64,
        pub K: Vec<f64>,
        pub r: f64,
        pub sigma: f64,
        pub T: f64
    }
    
    fn distribution1(params: &BlackScholesFormulaParams) -> f64 {
        ((params.S / params.K).ln() + ((params.r + params.sigma.powf(2.0)/2.0) * params.T))/(params.sigma * params.T.sqrt())
    }
    
    fn distribution2(d1: f64, params: &BlackScholesFormulaParams) -> f64 {
        d1 - (params.sigma * params.T.sqrt())
    }
    
    fn erf(x: f64) -> f64 {
        // Constants
        let a1 =  0.254829592;
        let a2 = -0.284496736;
        let a3 =  1.421413741;
        let a4 = -1.453152027;
        let a5 =  1.061405429;
        let p  =  0.3275911;
    
        // Save the sign of x
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        let x = x.abs();
    
        // A&S formula 7.1.26
        let t = 1.0 / (1.0 + p * x);
        let y = ((((a5 * t + a4) * t) + a3) * t + a2) * t + a1;
        let erf = 1.0 - y * (-x * x).exp();
    
        sign * erf
    }
    
    fn cdf_normal(x: f64) -> f64 {
        0.5 * (1.0 + erf(x / (2.0_f64).sqrt()))
    }

    pub fn black_scholes_formula(params: BlackScholesFormulaParams) -> f64 {
        let d1 = distribution1(&params);
        let d2 = distribution2(d1, &params);
    
        let call_option = params.S * cdf_normal(d1) - params.K * (-params.r * params.T).exp() * cdf_normal(d2);
        call_option
    }

    #[tokio::main]
    pub async fn black_scholes_formula_multiple_strike_prices(params: BlackScholesFormulaStrikePriceStreamParams) -> Vec<f64> {
        use std::time::Instant;
        let now = Instant::now();
        let futures: Vec<_> = params.K.clone().into_iter().map(|strike_price| {
            let local_params = BlackScholesFormulaParams {
                S: params.S,
                K: strike_price,
                r: params.r,
                sigma: params.sigma,
                T: params.T,
            };
            
            task::spawn(async move {
                let d1 = distribution1(&local_params);
                let d2 = distribution2(d1, &local_params);
                local_params.S * cdf_normal(d1) - strike_price * (-local_params.r * local_params.T).exp() * cdf_normal(d2)
            })
        }).collect();

    
        let results: Vec<_> = futures::future::join_all(futures).await.into_iter().map(|res| res.unwrap()).collect();
        let elapsed = now.elapsed();
        println!("Elapsed time (Synchronous requests): {:.3?}", elapsed);
        results
    }

    pub fn black_scholes_formula_multiple_strike_prices_async(params: BlackScholesFormulaStrikePriceStreamParams) -> Vec<f64> {
        use std::time::Instant;
        let now = Instant::now();
        let results: Vec<_> = params.K.clone().into_iter().map(|strike_price| {
            let local_params = BlackScholesFormulaParams {
                S: params.S,
                K: strike_price,
                r: params.r,
                sigma: params.sigma,
                T: params.T,
            };
            
            let d1 = distribution1(&local_params);
            let d2 = distribution2(d1, &local_params);
            local_params.S * cdf_normal(d1) - strike_price * (-local_params.r * local_params.T).exp() * cdf_normal(d2)
        }).collect();
        let elapsed = now.elapsed();
        println!("Elapsed time (Concurrent Requests): {:.3?}", elapsed);
        results
    }
}