use chrono::NaiveDateTime;
use rig::{
    embeddings::{EmbedError, TextEmbedder},
    Embed,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::From, fmt::Debug};

#[derive(Debug, thiserror::Error)]
#[error("Portfolio error: {0}")]
pub struct PortfolioError(pub String);

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Asset {
    pub token: StringContractAddress,
    #[serde(rename = "balanceUSD")]
    pub balance: f64,
}

#[derive(Debug, Deserialize, Serialize, Default, JsonSchema, Clone, PartialEq, Eq, Hash)]
pub struct Token {
    pub name: String,
    pub address: StringContractAddress,
    #[serde(rename = "priceUSD")]
    pub price: Price,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Default, Eq, Hash)]
pub struct StringContractAddress(pub String);

impl StringContractAddress {
    pub fn from(str: &str) -> StringContractAddress {
        Self(str.to_string())
    }
}

impl From<HashMap<String, Token>> for Token {
    fn from(map: HashMap<String, Token>) -> Token {
        // Assuming you want to combine all tokens or take the first one
        if let Some((_, token)) = map.into_iter().next() {
            token
        } else {
            eprint!("no token found");
            Token::default()
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ComputeError {
    //    #[error("Missing price data for token")]
    //    MissingPrice,
    //    #[error("Pool TVL too low")]
    //    LowTVL,
    #[error("Invalid pool data")]
    InvalidPool,
    //  #[error("Missing liquidity data")]
    //    MissingLiquidity,
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Debug, Clone)]
pub struct ProtocolYield {
    pub token: Token,
    pub apy: f64,
    pub tvl: f64,
    pub volume_24h: f64,
    pub risk_score: f64,
    pub pool_type: PoolType,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct Portfolio {
    wallet_address: StringContractAddress,
    assets: Vec<Asset>,
    total_value: f64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct YieldAnalyzer {
    pub portfolio_data: Vec<Asset>,
    pub yields_data: Vec<ProtocolYield>,
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Debug, Clone)]
pub enum PoolType {
    #[default]
    Stable,
    Volatile,
    Degen,
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct TwitterInsight {
    pub tweet_text: String,
    pub author: String,
    #[serde(serialize_with = "serialize_datetime")]
    pub timestamp: NaiveDateTime,
    /// This is String in the error i just sent.
    pub strategy_type: String,
    pub protocol_mentioned: String,
    pub sentiment: i32,
    pub engagement_score: i32,
}

impl Embed for TwitterInsight {
    fn embed(&self, embedder: &mut TextEmbedder) -> Result<(), EmbedError> {
        // Create a rich context string that combines all relevant fields
        let context = format!(
            "Strategy: {} \n\
            Protocol: {} \n\
            Tweet: {} \n\
            Author: {} \n\
            Sentiment: {} \n\
            Engagement: {}",
            self.strategy_type,
            self.protocol_mentioned,
            self.tweet_text,
            self.author,
            self.sentiment,
            self.engagement_score
        );

        // Add the context to be embedded
        embedder.embed(context);
        Ok(())
    }
}

fn serialize_datetime<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let formatted = datetime.format("%Y-%m-%d %H:%M:%S UTC").to_string();
    serializer.serialize_str(&formatted)
}

#[derive(Debug, Clone, Serialize)]
pub struct DefiKnowledge {
    id: String,
    content: String,
}

impl Embed for DefiKnowledge {
    fn embed(&self, embedder: &mut TextEmbedder) -> Result<(), EmbedError> {
        // embed the content field
        embedder.embed(self.content.clone());
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize, Default, JsonSchema, Clone, PartialEq, Eq, Hash)]
pub struct Price {
    integral: u64,
    fractional: u64,
    decimals: u8, // Store precision level (e.g., 6 for 6 decimal places)
}

impl Price {
    pub fn new(integral: u64, fractional: u64, decimals: u8) -> Self {
        Self {
            integral,
            fractional,
            decimals,
        }
    }

    pub fn from_f64(value: f64, decimals: u8) -> Self {
        let multiplier = 10u64.pow(decimals as u32);
        let total = (value * multiplier as f64) as u64;
        Self {
            integral: total / multiplier,
            fractional: total % multiplier,
            decimals,
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.integral as f64 + (self.fractional as f64 / 10f64.powi(self.decimals as i32))
    }
}
