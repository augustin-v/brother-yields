use crate::{agent_tools::yield_analyzer::AnalyzerTool, backend::Backend, types::ProtocolYield};

use rig::{
    agent::{Agent, AgentBuilder},
    completion::{Chat, CompletionModel, Message, Prompt, PromptError},
    loaders::FileLoader,
};

#[derive(Clone)]
pub struct Tools {
    pub analyzer_tool: AnalyzerTool,
}

impl Tools {
    pub fn new(yields_data: Vec<ProtocolYield>) -> Self {
        Self {
            analyzer_tool: AnalyzerTool { yields_data },
        }
    }
}

pub struct Navigator<M: CompletionModel> {
    navigator: Agent<M>,
    defiproman: Agent<M>,
    pub chat_history: Vec<rig::completion::Message>,
    tools: Tools,
}

impl<M: CompletionModel> Navigator<M> {
    pub fn new(nav_model: M, defaigent_model: AgentBuilder<M>, tools: Tools) -> Self {
        Self {
            navigator: agent_build(nav_model.clone()).expect("Failed building navigator"),
            defiproman: super::lp_pro_man::proman_agent_build(defaigent_model, tools.clone())
                .expect("Failed building defiproman"),
            chat_history: vec![],
            tools,
        }
    }

    pub async fn process_prompt(
        &mut self,
        prompt: &str,
    ) -> Result<String, rig::completion::PromptError> {
        self.chat_history.push(Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        });

        let refined_prompt = self.navigator.prompt(prompt).await?;
        println!("{refined_prompt}");

        let response = self
            .defiproman
            .chat(&refined_prompt, self.chat_history.clone())
            .await
            .map_err(|e| {
                rig::completion::PromptError::CompletionError(
                    rig::completion::CompletionError::ResponseError(e.to_string()),
                )
            })?;

        self.chat_history.push(Message {
            role: "assistant".to_string(),
            content: response.clone(),
        });

        Ok(response)
    }

    pub fn update_chat_history(&mut self, content: &str) {
        self.chat_history.push(Message {
            role: "system".to_string(),
            content: content.to_string(),
        });
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

pub async fn launch<M: CompletionModel>(backend: &Backend<M>) -> Result<(), anyhow::Error> {
    let nav_agent = backend
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
        .preamble(
            "You are a navigator in the Brother Yield project, made for assisting thproperly.",
        )
        .build();

    Ok(agent)
}
