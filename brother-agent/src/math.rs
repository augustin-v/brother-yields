pub const STARKNET_TVL_ESTIMATE: f64 = 231984602.0;

pub fn calculate_risk_score(tvl: f64, volume_24h: f64, price_change_24h: f64) -> f64 {
    // TVL factor (0-40 points, lower TVL = higher risk)
    let tvl_score = 40.0 * (1.0 - (1000000.0 / tvl).min(1.0));

    // Volume factor (0-30 points, lower volume = higher risk)
    let volume_score = 30.0 * (1.0 - (100000.0 / volume_24h).min(1.0));

    // Volatility factor (0-30 points, higher price change = higher risk)
    let volatility_score = 30.0 * (price_change_24h.abs() / 100.0).min(1.0);

    // Final score (0-100, where 0 is highest risk and 100 is lowest risk)
    100.0 - (tvl_score + volume_score + volatility_score)
}
