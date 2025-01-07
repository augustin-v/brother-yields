use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use starknet::core::types::Felt;

#[derive(Debug, thiserror::Error)]
#[error("Portfolio error: {0}")]
pub struct PortfolioError(pub String);

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub token: Felt,
    #[serde(rename = "balanceUSD")]
    pub balance: f64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone)]
pub struct StringContractAddress(pub String);
