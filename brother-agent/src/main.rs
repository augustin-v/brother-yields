use std::collections::HashMap;

use anyhow::{Error, Ok};
use dotenv::dotenv;
use rig::providers::openai;
use rig::{completion::Prompt, loaders::FileLoader, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokens::fetch_all_tokens;
use types::{PortfolioError, StringContractAddress, Token};

mod market;
mod math;
mod query;
mod tokens;
mod types;

#[derive(Deserialize, Serialize, JsonSchema, Default)]
struct ProtocolYield {
    token: Token,
    apy: f64,
    tvl: f64,
    risk_score: f64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct Portfolio {
    wallet_address: StringContractAddress,
    assets: Vec<Asset>,
    total_value: f64,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct Asset {
    token: StringContractAddress,
    amount: f64,
    current_protocol: Option<StringContractAddress>,
}

#[derive(Deserialize, Serialize, JsonSchema)]
struct YieldAnalyzer {
    portfolio_data: Vec<Asset>,
}

impl YieldAnalyzer {
    /// supported for now: "STRK", "BROTHER", "ETH"
    async fn get_yields_data() -> Result<Vec<ProtocolYield>, Error> {
        // TODO: call fetch_all_tokens -> compute the apy etc based on that -> store it in the protocol yield
        let (tokens, market_data) = fetch_all_tokens().await;
        for (token, market) in tokens.iter().zip(market_data.iter()) {
            println!("Token {} has price ${} and 24h volume ${}", 
                token.name, 
                market.price, 
                market.volume_24h);
        }
        Ok(vec![ProtocolYield::default()]).map_err(|e| Error::context(e, "err"))
    }
}

//impl Tool for YieldAnalyzer {
//    const NAME: &'static str = "analyze_yield";
//    type Error = PortfolioError;
//    type Args = Portfolio;
//    type Output = Vec<ProtocolYield>;
//
//    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
//        rig::completion::ToolDefinition {
//            name: Self::NAME.to_string(),
//            description: "Analyzes portfolio and suggests optimal yield strategies on Starknet"
//                .to_string(),
//            parameters: json!({
//                "type": "object",
//                "properties": {
//                    "wallet_address": {"type": "string"},
//                    "assets": {
//                        "type": "array",
//                        "items": {
//                            "type": "object",
//                            "properties": {
//                                "token": {"type": "string"},
//                                "amount": {"type": "number"},
//                                "current_protocol": {"type": "string"}
//                            }
//                        }
//                    }
//                }
//            }),
//        }
//    }
//
//    async fn call(&self, portfolio: Self::Args) -> Result<Self::Output, Self::Error> {
//        // call llm tool
//    }
//}

#[tokio::main]
async fn main() {
    dotenv().expect("failed to load .env");
//    let openai_client = openai::Client::from_env();
    
    
    
    
    YieldAnalyzer::get_yields_data().await;




//    let yield_agent = openai_client
//        .agent(openai::GPT_4O)
//        .preamble("You are YieldAI, an expert DeFi yield optimization assistant on Starknet.")
//        .tool(YieldAnalyzer {
//            portfolio_data: vec![],
//        })
//        .build();
//
//    let result = yield_agent
//        .prompt("Are you doing good?")
//        .await
//        .expect("Failed prompting gpt");
//    println!("gpt: {}", result);
}
