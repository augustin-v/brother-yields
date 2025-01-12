
use backend::Backend;
use dotenv::dotenv;
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

    let openai_client =
    rig::providers::openai::Client::new(&openai_api);
    
    // Initiate agents and backend
    let model = openai_client.completion_model("gpt-4o-mini");
    let backend = Backend::new();
    let server_task = tokio::spawn(async move {backend.start(model).await.expect("didnt start")});
    
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Fetch market data  Compute into yields data: apy, risk score etc...
    let _all_data = YieldAnalyzer::get_yields_data().await.expect("no data");
    server_task.await.expect("Server crashed unexpectedly");
}
