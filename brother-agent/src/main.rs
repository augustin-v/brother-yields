use std::collections::HashMap;

use anyhow::{Error, Ok};
use dotenv::dotenv;
use rig::providers::openai;
use rig::{completion::Prompt, loaders::FileLoader, tool::Tool};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokens::fetch_all_tokens;
use tracing::info;
use types::{ProtocolYield, StringContractAddress, YieldAnalyzer};
use utils::call_felt_2_usize_contract;
use backend::Backend;
use std::sync::atomic::Ordering;

mod market;
mod math;
mod tokens;
mod types;
mod utils;
mod agent_tools;
mod agents;
mod backend;


#[tokio::main]
async fn main() {
    dotenv().expect("failed to load .env");
    let  backend = Backend::new();
    info!("Backend {} url {:?}", backend.is_active.load(Ordering::SeqCst), backend.clone().listener_addr.read());
    backend.start().await.expect("didnt start");
    // Fetch market data  Compute into yields data: apy, risk score etc...
    let all_data = YieldAnalyzer::get_yields_data().await.expect("no data");

//    let openai_client = openai::Client::from_env();
//
//    let yield_agent = openai_client
//        .agent(openai::GPT_4O)
//        .preamble("You are YieldAI, an expert DeFi yield optimization assistant on Starknet.")
//        .tool(YieldAnalyzer {
//            portfolio_data: vec![],
//            yields_data: all_data,
//        })
//        .build();
//
//    let result = yield_agent
//        .prompt("Are you doing good?")
//        .await
//        .expect("Failed prompting gpt");
//    println!("gpt: {}", result);
}
