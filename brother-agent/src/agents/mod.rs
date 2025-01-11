

use anyhow::{Context, Error};
use rig::{
    agent::Agent,
    completion:: Prompt,
    providers::openai::CompletionModel
};
use std::fs;
use std::sync::Arc;
use std::{ convert::From, fmt::Debug, path::Path};
use tokio::sync::Mutex;

pub mod navigator;

#[derive(Clone)]
pub struct AgentState {
    pub navigator: Arc<Mutex<BrotherAgent>>,
}

/// Agents have distinct roles
#[derive(Debug)]
pub enum AgentRole {
    Navigator,
    Analyzer,
}

pub struct BrotherAgent {
    pub agent: Arc<Agent<CompletionModel>>,
    pub job: AgentRole,
}

impl BrotherAgent {
    pub fn new(self, job: AgentRole) -> Self {
        Self {
            agent: self.agent,
            job,
        }
    }

    pub fn from(agent: Agent<CompletionModel>, job: AgentRole) -> Self {
        Self {
            agent: Arc::new(agent),
            job,
        }
    }

    pub async fn proccess_message(&self, message: &str) -> Result<String, Error> {
        self.agent
            .prompt(message)
            .await
            .map_err(anyhow::Error::from)
    }

    pub fn load_md_content<P: AsRef<Path> + Debug>(file_path: P) -> Result<String, Error> {
        fs::read_to_string(file_path.as_ref())
            .with_context(|| format!("Failed to read markdown file: {:?}", file_path))
    }
}
