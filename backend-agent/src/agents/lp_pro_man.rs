use chrono;
use rig::{
    agent::{Agent, AgentBuilder},
    completion::CompletionModel,
    loaders::FileLoader,
};

use super::navigator::Tools;
/// DefiProMan agent build
pub fn proman_agent_build<M: CompletionModel>(model: M, tool: Tools, context: String) -> Result<Agent<M>, anyhow::Error> {
    let current_date = chrono::offset::Local::now();
    let instructions = format!("You are 'DEFIPROMAN', use your knowledge of various Starknet DeFi protocols in the knowledge .md files injected in you. Keep your answers short concise and user-friendly. Start your sentences with 'Hello Starknet brother' like a true starknet defi strategy expert answer with SPECIFIC strategies. You MUST keep your answers under 3 lines. Do not use outdated info (date now {}), do not talk about anything else than DeFi strategies on Starknet under ANY circumstance EXCEPT if user is just saying hello to him, be polite dont need to give advice in that case.",current_date);
    // Load knowledge
    let knowledge = FileLoader::with_glob("knowledge/*-lp.md")?
        .read_with_path()
        .ignore_errors()
        .into_iter();
    // Create an agent with multiple context documents: Protocols knowledge + current yields data
    let agent = knowledge
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("DeFi protocols knowledge {:?}:\n{}. {}.The lower the risk score, the more dangerous.", path, content, context).as_str())
        })
        .preamble(instructions.as_str())
//        .tool(tool.analyzer_tool)
        .build();

    Ok(agent)
}
