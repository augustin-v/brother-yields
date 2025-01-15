use agents::navigator::Tools;
use backend::Backend;
use dotenv::dotenv;
use types::YieldAnalyzer;
use insights::get_insights_context;

mod agent_tools;
mod agents;
mod backend;
mod insights;
mod market;
mod math;
mod tokens;
mod types;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().expect("failed to load .env");

    let openai_api = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY undefined in .env");
    let yields_data = YieldAnalyzer::get_yields_data()
        .await
        .expect("no yield data");

    let openai_client = rig::providers::openai::Client::new(&openai_api);

    // Initiate agents, tools and backend
    let tools = Tools::new(yields_data.clone());
    let x_insight = get_insights_context().await.expect("Failed getting twitter insights");
    let context = format!("{} \n\n {}", crate::agent_tools::yield_analyzer::format_yields_data(yields_data.clone()), x_insight);
    let model = openai_client.completion_model("gpt-4o-mini");
    let backend = Backend::new(yields_data);
    let server_task = tokio::spawn(async move {
        backend
            .start(model, tools, context)
            .await
            .expect("didnt start")
    });

    server_task.await.expect("Server crashed unexpectedly");
}
