use statrs::distribution::{Normal, Univariate};
use std::f64::consts::E;

fn black_scholes_call(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = (f64::ln(S / K) + (r + 0.5 * sigma * sigma) * T) / (sigma * f64::sqrt(T));
    let d2 = d1 - sigma * f64::sqrt(T);
    let norm = Normal::new(0.0, 1.0).unwrap();
    S * norm.cdf(d1) - K * E.powf(-r * T) * norm.cdf(d2)
}

fn black_scholes_put(S: f64, K: f64, T: f64, r: f64, sigma: f64) -> f64 {
    let d1 = (f64::ln(S / K) + (r + 0.5 * sigma * sigma) * T) / (sigma * f64::sqrt(T));
    let d2 = d1 - sigma * f64::sqrt(T);
    let norm = Normal::new(0.0, 1.0).unwrap();
    K * E.powf(-r * T) * norm.cdf(-d2) - S * norm.cdf(-d1)
}
