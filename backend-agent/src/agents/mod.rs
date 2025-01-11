pub mod navigator;

use anyhow::{Context, Error, Ok};
use navigator::navigator_build;
use parking_lot:: RwLock;
use rig::{
    agent::Agent,
    completion::{CompletionModel, Prompt},
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;
use std::{collections::HashMap, convert::From, fmt::Debug, path::Path};

#[derive(Clone)]
pub struct AgentState<M: CompletionModel> {
    navigator: Arc<RwLock<Option<BrotherAgent<M>>>>,
    //add new agents here
}


pub struct BrotherAgent<M: CompletionModel> {
    pub agent: Arc<Agent<M>>,
    pub job: AgentRole,
}

impl<M: CompletionModel> AgentState<M> {
    pub async fn build_navigator(model: M) -> Result<Self, anyhow::Error> {
        let navigator = Arc::new(RwLock::new(Some(BrotherAgent::from(navigator_build(model).await?, AgentRole::Navigator))));
        Ok(Self { navigator })
    }
}

impl<M: CompletionModel> BrotherAgent<M> {
    pub fn new(self, job: AgentRole) -> Self {
        Self {
            agent: self.agent,
            job,
        }
    }

    pub fn from(agent: Agent<M>, job: AgentRole) -> Self {
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

/// Agents have distinct roles
#[derive(Debug)]
pub enum AgentRole {
    Navigator,
    Analyzer,
}