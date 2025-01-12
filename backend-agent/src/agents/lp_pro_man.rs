use rig::{
    agent::{Agent, AgentBuilder}, completion::{Chat, CompletionModel}, loaders::FileLoader
};

/// DefiProMan agent build
pub fn proman_agent_build<M: CompletionModel>(model: M) -> Result<Agent<M>, anyhow::Error> {
    // Load knowledge
    let knowledge = FileLoader::with_glob("knowledge/*-lp.md")?
        .read_with_path()
        .ignore_errors()
        .into_iter();
    // Create an agent with multiple context documents
    let agent = knowledge
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("DeFi protocols knowledge {:?}:\n{}", path, content).as_str())
        }).preamble("You are 'DEFIPROMAN', use your knowledge of various Starknet DeFi protocols in the knowledge .md files injected in you. Keep your answers short concise and user-friendly. Start your sentences with 'Hello Starknet brother' like a true starknet defi strategy expert.")
        .build();

    Ok(agent)
}
