use crate::backend::Backend;
use anyhow::Error;
use rig::{
    agent::{Agent, AgentBuilder}, loaders::FileLoader, providers::openai::{self, CompletionModel, GPT_4O}
};

pub async fn launch(backend: &Backend) -> Result<(), Error> {
    let nav_agent = backend.agent_state.clone().expect("No agent available").navigator;
    let result = nav_agent.lock().await.proccess_message("Hi from space.").await.expect("Failed processing message");
    println!("GOOD,{}", result);

    Ok(())
}

pub async fn agent_build(api_key: String) -> Result<Agent<CompletionModel>, anyhow::Error> {
    let openai_client =
        openai::Client::new(&api_key);

    let model = openai_client.completion_model(GPT_4O);

    // Load in all the rust examples
    let examples = FileLoader::with_glob("brother-agent/agents/*.rs")?
        .read_with_path()
        .ignore_errors()
        .into_iter();

    // Create an agent with multiple context documents
    let agent = examples
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("Rust Example {:?}:\n{}", path, content).as_str())
        }).preamble("You are a navigator in the Brother Yield project, made for assisting the user with DeFi strategy optimization on Starknet. You are the mastermind with all the tools. Use them wisely to meet the user's expectations. Do not answer requests unrelated to Starknet or DeFi strategies on Starknet under ANY circumstance. Keep your answer concise, no hyperbole allowed. Do not forget that you are nothing but a NAVIGATOR, your role is to maintain the route and guide the user to the true expert agents. Your reply must be short under 2 lines. If asked about Yield farming in the contet of liquidity providing guide them to 'PROLIQUIDITYMAN', the know it all.")
        .build();

    Ok(agent)
}
