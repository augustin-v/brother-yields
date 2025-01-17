use rig::{
    agent::{Agent, AgentBuilder},
    completion::CompletionModel,
    loaders::FileLoader,
};

use super::navigator::Tools;
/// DefiProMan agent build
pub fn proman_agent_build<M: CompletionModel + 'static>(
    model: AgentBuilder<M>,
    tool: Tools<M>,
) -> Result<Agent<M>, anyhow::Error> {
    // Load knowledge
    let knowledge = FileLoader::with_glob("knowledge/*-lp.md")?
        .read_with_path()
        .ignore_errors()
        .into_iter();
    // Create an agent with multiple context documents: Protocols knowledge + current yields data
    let agent = knowledge
        .fold(model, |builder, (path, content)| {
            builder.context(format!("DeFi protocols knowledge {:?}:\n{}.", path, content).as_str())
        })
        .tool(tool.portfolio_tool)
        .build();

    Ok(agent)
}
