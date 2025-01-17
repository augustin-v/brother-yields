use crate::{
    agent_tools::{portfolio::PortfolioFetch, yield_analyzer::AnalyzerTool},
    backend::{AppState, Backend},
    types::ProtocolYield,
};
use anyhow::Ok;
use rig::{
    agent::{Agent, AgentBuilder},
    completion::{Chat, CompletionModel, Message, Prompt},
    loaders::FileLoader,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

#[derive(Clone)]
pub struct Tools<M: CompletionModel> {
    pub _analyzer_tool: AnalyzerTool,
    pub portfolio_tool: PortfolioFetch<M>,
}

impl<M: CompletionModel> Tools<M> {
    pub fn new(yields_data: Vec<ProtocolYield>, appstate: Arc<Mutex<AppState<M>>>) -> Self {
        Self {
            _analyzer_tool: AnalyzerTool { yields_data },
            portfolio_tool: PortfolioFetch { appstate },
        }
    }
}

pub struct Navigator<M: CompletionModel> {
    navigator: Agent<M>,
    defiproman: Agent<M>,
    pub chat_history: Arc<Mutex<Vec<Message>>>,
    tools: Tools<M>,
}

impl<M: CompletionModel + 'static> Navigator<M> {
    pub fn new(nav_model: M, defaigent_model: AgentBuilder<M>, tools: Tools<M>) -> Self {
        Self {
            navigator: agent_build(nav_model.clone()).expect("Failed building navigator"),
            defiproman: super::lp_pro_man::proman_agent_build(defaigent_model, tools.clone())
                .expect("Failed building defiproman"),
            chat_history: Arc::new(Mutex::new(vec![])),
            tools,
        }
    }

    pub async fn process_prompt(
        &mut self,
        prompt: &str,
    ) -> Result<String, rig::completion::PromptError> {
        self.chat_history.lock().await.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let refined_prompt = self.navigator.prompt(prompt).await?;
        println!("{refined_prompt}");

        let response = self
            .defiproman
            .chat(&refined_prompt, self.chat_history.lock().await.clone())
            .await
            .map_err(|e| {
                rig::completion::PromptError::CompletionError(
                    rig::completion::CompletionError::ResponseError(e.to_string()),
                )
            })?;

        self.chat_history.lock().await.push(Message {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response).map_err(|e| {
            rig::completion::PromptError::CompletionError(
                rig::completion::CompletionError::ResponseError(e.to_string()),
            )
        })
    }

    pub async fn _update_chat_history(&mut self, content: &str) -> Result<(), anyhow::Error> {
        info!("Starting chat history update...");
        // Add debug info about the current state
        info!(
            "Current chat history length: {}",
            self.chat_history.lock().await.len()
        );

        self.chat_history.lock().await.push(Message {
            role: "system".to_string(),
            content: content.to_string(),
        });

        info!(
            "Message pushed to chat history. New length: {}",
            self.chat_history.lock().await.len()
        );
        Ok(())
    }
}

impl<M: CompletionModel> Chat for Navigator<M> {
    async fn chat(
        &self,
        prompt: &str,
        chat_history: Vec<rig::completion::Message>,
    ) -> Result<String, rig::completion::PromptError> {
        let refined_prompt = self.navigator.prompt(prompt).await?;

        self.defiproman
            .chat(&refined_prompt, chat_history.clone())
            .await
    }
}

pub async fn launch<M: CompletionModel + 'static>(
    backend: &Backend<M>,
) -> Result<(), anyhow::Error> {
    let nav_agent = backend
        .app_state
        .lock()
        .await
        .agent_state
        .clone()
        .expect("No agent available")
        .navigator;
    let result = nav_agent
        .lock()
        .await
        .process_prompt("Hi from space.")
        .await
        .expect("Failed processing prompt");
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
        })
        .preamble("You are a navigator in the Brother Yield project, made for assisting the user with DeFi strategy optimization on Starknet. You have your own AI defi expert, called LiquidityProMan(LPM). So when user asks you a question you will be the middleman: refine the user prompt and use your refined version to prompt LPM. Keep your prompts shorter than 2 lines, start by 'brother defiproman {your_refined_prompt}'. ex: 'user:' 'hello' 'navigator': 'brother defiproman hello'. Be sure to rely the greetings properly.")
        .build();

    Ok(agent)
}
