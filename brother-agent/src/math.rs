use crate::{
    query::get_pools::GetPoolsPoolsDataPool, types::ComputeError, types::StringContractAddress,
};
use std::collections::HashMap;


pub const STARKNET_TVL_ESTIMATE: f64 = 231984602.0;
pub struct DefiSpringData {
    pub current_apr: f64,
    pub pool_address: StringContractAddress,
}

pub struct PoolMetrics {
    pub tvl_usd: f64,
    pub apy_base: f64,
    pub apy_reward: f64,
    pub tokens: (String, String),
}

pub fn calculate_pool_metrics(
    pool: &GetPoolsPoolsDataPool,
    prices: &HashMap<String, f64>,
    spring_data: Option<&DefiSpringData>,
) -> Result<PoolMetrics, ComputeError> {
    let tvl_usd = calculate_tvl(pool, prices)?;

    // Skip pools with low TVL
    if tvl_usd < 10_000.0 {
        return Err(ComputeError::LowTVL);
    }

    let apy_base = estimate_base_apy(pool)?;
    let apy_reward = match spring_data {
        Some(data) if data.pool_address.0 == pool.pool_address => data.current_apr,
        _ => 0.0,
    };

    Ok(PoolMetrics {
        tvl_usd,
        apy_base,
        apy_reward,
        tokens: (pool.token0.symbol.clone(), pool.token1.symbol.clone()),
    })
}

fn calculate_tvl(
    pool: &GetPoolsPoolsDataPool,
    prices: &HashMap<String, f64>,
) -> Result<f64, ComputeError> {
    if let Some(tvl_usd) = pool.total_value_locked_usd {
        return Ok(tvl_usd);
    }

    let tvl_token0 = pool
        .total_value_locked_token0
        .ok_or(ComputeError::MissingLiquidity)?;
    let tvl_token1 = pool
        .total_value_locked_token1
        .ok_or(ComputeError::MissingLiquidity)?;

    let price0 = prices
        .get(&pool.token0.symbol)
        .ok_or(ComputeError::MissingPrice)?;
    let price1 = prices
        .get(&pool.token1.symbol)
        .ok_or(ComputeError::MissingPrice)?;

    Ok(tvl_token0 * price0 + tvl_token1 * price1)
}

fn estimate_base_apy(pool: &GetPoolsPoolsDataPool) -> Result<f64, ComputeError> {
    let volume_usd = pool.volume_usd.ok_or(ComputeError::InvalidPool)?;
    let tvl_usd = pool
        .total_value_locked_usd
        .ok_or(ComputeError::InvalidPool)?;

    if tvl_usd > 0.0 {
        Ok((volume_usd * 365.0) / tvl_usd)
    } else {
        Ok(0.0)
    }
}

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
