use crate::agents::{AgentRole, BrotherAgent};
use anyhow::Error;
use rig::{
    agent::{Agent, AgentBuilder},
    completion::CompletionModel,
    loaders::FileLoader,
};
use std::env;

/// Role of the agentg
const ROLE: AgentRole = AgentRole::Navigator;

pub async fn launch<M: CompletionModel>(model: M) -> Result<(), Error> {
    let nav_agent = BrotherAgent::<M>::from(
        navigator_build(model).await.expect("Error building navigator agent"),
        ROLE,
    );


    Ok(())
}

pub async fn navigator_build<M: CompletionModel>(model: M) -> Result<Agent<M>, anyhow::Error> {

    // Load in all the rust examples
    let examples = FileLoader::with_glob("backend-agent/agents/*.rs")?
        .read_with_path()
        .ignore_errors()
        .into_iter();

    // Create an agent with multiple context documents
    let agent = examples
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("Rust Example {:?}:\n{}", path, content).as_str())
        }).preamble("You are a navigator in the Brother Yield project, made for assisting the user with DeFi strategy optimization on Starknet. You are the mastermind with all the tools. Use them wisely to meet the user's expectations. Do not answer requests unrelated to Starknet or DeFi strategies on Starknet under ANY circumstance.")
        .build();

    
    Ok(agent)
}
// Prompt the agent and print the response
//    let response = agent
//        .prompt("Which rust example is best suited for the operation 1 + 2")
//        .await?;

//    println!("{}", response);
