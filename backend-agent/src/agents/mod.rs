use navigator::Navigator;
use rig::completion::CompletionModel;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod lp_pro_man;
pub mod navigator;

#[derive(Clone)]
pub struct AgentState<M: CompletionModel> {
    pub navigator: Arc<Mutex<Navigator<M>>>,
}
