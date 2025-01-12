

use navigator::Navigator;
use rig::completion::CompletionModel;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod navigator;
pub mod lp_pro_man;

#[derive(Clone)]
pub struct AgentState<M: CompletionModel> {
    pub navigator: Arc<Mutex<Navigator<M>>>,
}