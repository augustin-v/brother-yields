use crate::backend::Backend;
use anyhow::{Error, Ok};
use rig::{
    agent::{Agent, AgentBuilder}, completion::{Chat, CompletionModel, Prompt}, loaders::FileLoader
};


pub struct Navigator<M: CompletionModel> {
    navigator: Agent<M>,
    defiproman: Agent<M>,
    pub chat_history: Vec<rig::completion::Message>
}

impl<M: CompletionModel> Navigator<M> {
    pub fn new(model: M) -> Self {
        Self {
            navigator: agent_build(model.clone()).expect("Failed building navigator"),
            defiproman: super::lp_pro_man::proman_agent_build(model).expect("Failed building defiproman"),
            chat_history: vec![]
        }
    }

    pub async fn process_prompt(&self, prompt: &str) -> Result<String, rig::completion::PromptError> {

        let refined_prompt = self.navigator.prompt(prompt).await?;
        println!("{refined_prompt}");

        self.defiproman.prompt(&refined_prompt).await
    }
}

impl<M: CompletionModel> Chat for Navigator<M> {
    async fn chat(
            &self,
            prompt: &str,
            chat_history: Vec<rig::completion::Message>,
        ) -> Result<String, rig::completion::PromptError> {
            let refined_prompt = self.navigator.prompt(prompt).await?;


            self.defiproman.chat(&refined_prompt, chat_history.clone()).await
    }
}

pub async fn launch<M: CompletionModel>(backend: &Backend<M>) -> Result<(), Error> {
    let nav_agent = backend.agent_state.clone().expect("No agent available").navigator;
   // let result = nav_agent.lock().await.proccess_message("Hi from space.").await.expect("Failed processing message");
   // println!("GOOD,{}", result);

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
        }).preamble("You are a navigator in the Brother Yield project, made for assisting the user with DeFi strategy optimization on Starknet. You have your own AI assistant, called LiquidityProMan(LPM). So when user asks you a question, use LPM as the middleman then reply to user, basically refine the user prompt and use your refined version to prompt your assistant. Keep your prompts shorter than 2 lines, start by 'brother defiproman {your_refined_prompt}'. Only about DeFi on starknet. Do not talk about anything else than DeFi strategies on Starknet under ANY circumstance. ")
        .build();

    Ok(agent)
}
