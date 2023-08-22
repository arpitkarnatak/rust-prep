use std::String;

pub mod sharpe_ratio_ex {
    pub struct Asset {
        pub name: String,
        pub expected_return: f64,
        pub standard_deviation: f64,
    }

    pub struct OptimizePortfolioFunctionParams {
        pub assets: &Vec<Asset>,
        pub covariance_matrix: Vec<Vec<f64>>,
        pub risk_free_rate: f64,
    }

    struct SharpeRatioParams {
        expected_return: f64,
        risk_free_return: f64,
        standard_deviation: f64
    }
    
    fn sharpe_ratio(params: SharpeRatioParams) -> f64 {
        (params.expected_return - params.risk_free_return)/params.standard_deviation
    }

    pub fn optimize_portfolio(params: OptimizePortfolioFunctionParams) {

    }
}