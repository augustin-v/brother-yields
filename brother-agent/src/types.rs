use anyhow::{Context, Error};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::From, fmt::Debug, path::Path};
use rig::{agent::Agent, completion::{CompletionModel, Prompt}};
use std::sync::Arc;
use std::fs;

pub struct BrotherAgent<M: CompletionModel> {
    pub agent: Arc<Agent<M>>,
    pub job: AgentRole,
}

impl<M: CompletionModel> BrotherAgent<M> {
    pub fn new(self, job: AgentRole) -> Self {
        Self {
            agent: self.agent,
            job
        }
    }

    pub fn from(agent: Agent<M>, job: AgentRole) -> Self {
        Self {
            agent: Arc::new(agent),
            job
        }
    }

    pub async fn proccess_message(&self, message: &str) -> Result<String, Error> {
        self.agent.prompt(message).await.map_err(anyhow::Error::from)
    }

    pub fn load_md_content<P: AsRef<Path> +Debug>(file_path: P) -> Result<String, Error> {
        fs::read_to_string(file_path.as_ref())
            .with_context(|| format!("Failed to read markdown file: {:?}", file_path))
    }
}

/// Agents have distinct roles
#[derive(Debug)]
pub enum AgentRole {
    Navigator,
    Analyzer,
}

#[derive(Debug, thiserror::Error)]
#[error("Portfolio error: {0}")]
pub struct PortfolioError(pub String);

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct Asset {
    pub token: StringContractAddress,
    #[serde(rename = "balanceUSD")]
    pub balance: f64,
}

#[derive(Debug, Deserialize, Serialize, Default, JsonSchema, Clone)]
pub struct Token {
    pub name: String,
    pub address: StringContractAddress,
    #[serde(rename = "priceUSD")]
    pub price: f64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, Clone, PartialEq, Default)]
pub struct StringContractAddress(pub String);

impl StringContractAddress {
    pub fn from(str: &str) -> StringContractAddress {
        Self { 0: str.to_string() }
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
    #[error("Missing price data for token")]
    MissingPrice,
    #[error("Pool TVL too low")]
    LowTVL,
    #[error("Invalid pool data")]
    InvalidPool,
    #[error("Missing liquidity data")]
    MissingLiquidity,
}

#[derive(Deserialize, Serialize, JsonSchema, Default, Debug)]
pub struct ProtocolYield {
    pub token: Token,
    pub apy: f64,
    pub tvl: f64,
    pub risk_score: f64,
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