
use backend::Backend;
use dotenv::dotenv;
use serde_json::json;
use tokens::fetch_all_tokens;
use tokio::time::Duration;
use types::{StringContractAddress, YieldAnalyzer};

mod agent_tools;
mod agents;
mod backend;
mod market;
mod math;
mod tokens;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().expect("failed to load .env");
    let openai_api = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY undefined in .env");
    let backend = Backend::new();
    let server_task = tokio::spawn(async move {backend.start(openai_api).await.expect("didnt start")});
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Fetch market data  Compute into yields data: apy, risk score etc...
    let _all_data = YieldAnalyzer::get_yields_data().await.expect("no data");
    server_task.await.expect("Server crashed unexpectedly");
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
