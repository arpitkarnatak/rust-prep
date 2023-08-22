use std::thread;
pub mod black_scholes {
    #[derive(Debug)]
    pub struct BlackScholesFormulaParams {
        pub S: f64,
        pub K: f64,
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

}