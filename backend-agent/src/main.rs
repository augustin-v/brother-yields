use agents::navigator::Tools;
use backend::Backend;
use dotenv::dotenv;
use insights::get_insights_context;
use rig::{
    embeddings::EmbeddingsBuilder, providers::openai::TEXT_EMBEDDING_3_SMALL,
    vector_store::in_memory_store::InMemoryVectorStore,
};
use types::YieldAnalyzer;
use utils::defipro_get_instr;
use crate::backend::messaging::ChatHistoryManager;
use std::env;


mod agent_tools;
mod agents;
mod backend;
mod insights;
mod market;
mod math;
mod tokens;
mod types;
mod utils;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    dotenv().ok();

    let openai_api = std::env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set in environment");
    let yields_data = YieldAnalyzer::get_yields_data()
        .await
        .expect("no yield data");

    let openai_client = rig::providers::openai::Client::new(&openai_api);

    // Initiate agents, tools and backend
    let (_, x_insight) = get_insights_context()
        .await
        .expect("Failed getting twitter insights");

    let nav_model = openai_client.completion_model("gpt-4o-mini");

    let defaigent_embd_model = openai_client.embedding_model(TEXT_EMBEDDING_3_SMALL);
    let embeddings = EmbeddingsBuilder::new(defaigent_embd_model.clone())
        .documents(x_insight.clone())
        .expect("Failed embedding Vec<TwitterInsight>")
        .build()
        .await
        .expect("Failed building defaiproman");

    let vector_store = InMemoryVectorStore::from_documents(embeddings);
    let index = vector_store.index(defaigent_embd_model);

    let defaigent_model = openai_client
        .agent("gpt-4o-mini")
        .dynamic_context(4, index)
        .preamble(&defipro_get_instr())
        .temperature(0.3);

    let (manager, receiver) = ChatHistoryManager::new();


    let backend = Backend::new(yields_data.clone(), manager);
    let tools = Tools::new(yields_data, backend.app_state.clone());
    let server_task = tokio::spawn(async move {
        backend
            .start(nav_model, defaigent_model, tools, receiver)
            .await
            .expect("didnt start")
    });

    server_task.await.expect("Server crashed unexpectedly");
}
