use models::BlackScholesModel::*;

enum Type {
    Call,
    Put,
}


#[derive(Clone)]
struct Option{
    spot_price: f64,
    strike_price: f64,
    risk_free_rate: f64,
    volatility: f64,
    time_maturity: u8,
    option_type: Type,
}

impl Option {
    pub fn new(
        spot_price: f64,
        strike_price: f64,
        risk_free_rate: f64,
        volatility: f64,
        time_maturity: u8,
        option_type: Type,
    ) -> Self {
        Self {
            spot_price,
            strike_price,
            risk_free_rate,
            volatility,
            time_maturity,
            option_type: match option_type.as_str() {
                "call" => Type::Call,
                "put" => Type::Put,
                _ => panic!("Invalid option type"),
            },
        }
    }

    pub fn calculate_price(&self) -> f64 {
        match self.option_type {
            Type::Call => 
                return black_scholes_call(spot_price, strike_price, time_maturity, risk_free_rate, volatility),
            Type::Put => 
                return black_scholes_call(spot_price, strike_price, time_maturity, risk_free_rate, volatility),
        }
    }
}