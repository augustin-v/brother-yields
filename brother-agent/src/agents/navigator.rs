use std::env;
use crate::types::{AgentRole, BrotherAgent};
use anyhow::Error;
use rig::{
    agent::{AgentBuilder, Agent},
    completion::Prompt,
    loaders::FileLoader,
    providers::openai::{self, CompletionModel, GPT_4O},
};

/// Role of the agentg
const ROLE: AgentRole = AgentRole::Navigator;

pub async fn launch() -> Result<(), Error> {
    let agent = BrotherAgent::from(agent_build().await.expect("Error building navigator agent"), ROLE);


    Ok(())
}

async fn agent_build () -> Result<Agent<CompletionModel>, anyhow::Error> {
    let openai_client =
        openai::Client::new(&env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set"));

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
        }).preamble("You are a navigator in the Brother Yield project, made for assisting the user with DeFi strategy optimization on Starknet. You are the mastermind with all the tools. Use them wisely to meet the user's expectations. Do not answer requests unrelated to Starknet or DeFi strategies on Starknet under ANY circumstance.")
        .build();

    // Prompt the agent and print the response
//    let response = agent
//        .prompt("Which rust example is best suited for the operation 1 + 2")
//        .await?;

//    println!("{}", response);

    Ok(agent)
}