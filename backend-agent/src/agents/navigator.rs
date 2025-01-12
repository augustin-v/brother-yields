use crate::backend::Backend;
use anyhow::Error;
use rig::{
    agent::{Agent, AgentBuilder}, loaders::FileLoader, completion::CompletionModel
};


pub struct Navigator<M: CompletionModel> {
    navigator: Agent<M>,
    defiproman: Agent<M>
}

impl<M: CompletionModel> Navigator<M> {
    pub fn new(model: M) -> Self {
        Self {
            navigator: agent_build(model.clone()).expect("Failed building navigator"),
            defiproman: super::lp_pro_man::proman_agent_build(model).expect("Failed building defiproman")
        }
    }
}

pub async fn launch(backend: &Backend) -> Result<(), Error> {
    let nav_agent = backend.agent_state.clone().expect("No agent available").navigator;
    let result = nav_agent.lock().await.proccess_message("Hi from space.").await.expect("Failed processing message");
    println!("GOOD,{}", result);

    Ok(())
}

pub fn agent_build<M: CompletionModel>(model: M) -> Result<Agent<M>, anyhow::Error> {

    // Load in all the rust examples
    let examples = FileLoader::with_glob("agents/*.rs")?
        .read_with_path()
        .ignore_errors()
        .into_iter();
    // Create an agent with multiple context documents
    let agent = examples
        .fold(AgentBuilder::new(model), |builder, (path, content)| {
            builder.context(format!("Your agents knowledge {:?}:\n{}", path, content).as_str())
        }).preamble("You are a navigator in the Brother Yield project, made for assisting the user with DeFi strategy optimization on Starknet. You are the mastermind with all the tools. Use them wisely to meet the user's expectations. Do not answer requests unrelated to Starknet or DeFi strategies on Starknet under ANY circumstance. Keep your answer concise, no hyperbole allowed. Do not forget that you are nothing but a NAVIGATOR, your role is to maintain the route and guide the user to the true expert agents(Ony in defi context, if asked questions about code, Brother Yiled specializes in DeFi). Your reply must be short under 2 lines. If asked about Yield farming in the contet of liquidity providing guide them to 'DEFIPROMAN', the know it all. In short: make the user ask you question to redirect their question to an expert agent.")
        .build();

    Ok(agent)
}
