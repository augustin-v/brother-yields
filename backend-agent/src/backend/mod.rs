use crate::agents::navigator::navigator_build;
use crate::agents::{AgentRole, AgentState, BrotherAgent};
use crate::start::ProviderConfig;
use axum::{http::StatusCode, routing::post, Json, Router};
use parking_lot::RwLock;
use rig::completion::CompletionModel;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tracing::info;
use url::Url;

#[derive( Clone)]
pub struct Backend<M: CompletionModel> {
    pub is_active: Arc<AtomicBool>,
    pub listener_addr: Arc<RwLock<Option<Url>>>,
    pub provider: ProviderConfig,
    pub coingecko: String,
    pub agentstate: Arc<RwLock<Option<AgentState<M>>>>
}

#[derive(Deserialize)]
pub struct LaunchRequest {
    prompt: String,
}
#[derive(Serialize)]
pub struct LaunchResponse {
    pub status: String,
    pub message: String,
    pub agent_response: String,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    status: String,
    message: String,
}

impl<M: CompletionModel> Backend<M> {
    pub fn new(provider: ProviderConfig, coingecko: String) -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            listener_addr: Arc::new(RwLock::new(None)),
            provider,
            coingecko,
            agentstate: Arc::new(RwLock::new(None))
        }
    }

    pub async fn start(self) -> Result<(), anyhow::Error> {
                // Initialize agent state based on provider
        match &self.provider {
            ProviderConfig::OpenAI(api_key) => {
                let openai_client = rig::providers::openai::Client::new(api_key);
                let model = openai_client.completion_model(rig::providers::openai::GPT_4O);
                let agent_state = AgentState::build_navigator(model).await?;
                *self.agentstate.write() = Some(agent_state);
            },
            _ => return Err(anyhow::Error::msg("Unsupported provider"))
        }

        let state = Arc::new(self);
        let app = Router::new().route("/launch", post(launch_handler));
        match self.provider {
            ProviderConfig::OpenAI(str) => {
                // build the agent here
            },
            _ => panic!("No Completion provider in the config")
        }


        let listener = tokio::net::TcpListener::bind("127.0.0.1:5050")
            .await
            .expect("Listener failure");
        info!("Listener started on 127.0.0.1:300");

        self.is_active.store(true, Ordering::SeqCst);
        *self.listener_addr.write() = Some(Url::parse("http://127.0.0.1:5050/").unwrap());

        info!(
            "Backend {} url {:?}",
            self.is_active.load(Ordering::SeqCst),
            self.clone().listener_addr.read().as_ref().unwrap()
        );
        axum::serve(listener, app)
            .await
            .expect("axum serving failure");
        Ok(())
    }

    pub async fn launch_handler(self, 
        Json(request): Json<LaunchRequest>
    ) -> (StatusCode, Json<LaunchResponse>) {
        let nav_agent = BrotherAgent::<M>::from(
            navigator_build(self.agentstate).await.expect("Error building navigator agent"),
            AgentRole::Navigator,
        );
        
        match nav_agent.proccess_message(&request.prompt).await {
            Ok(response) => (
                StatusCode::OK,
                Json(LaunchResponse {
                    status: "success".to_string(),
                    message: "Agent successfully processed prompt".to_string(),
                    agent_response: response,
                }),
            ),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(LaunchResponse {
                    status: "error".to_string(),
                    message: e.to_string(),
                    agent_response: String::new(),
                }),
            ),
        }
    }
    
}

async fn launch_handler<M: CompletionModel>(
    Json(request): Json<LaunchRequest>
) -> (StatusCode, Json<LaunchResponse>) {
    let nav_agent = BrotherAgent::<M>::from(
        navigator_build().await.expect("Error building navigator agent"),
        AgentRole::Navigator,
    );
    
    match nav_agent.proccess_message(&request.prompt).await {
        Ok(response) => (
            StatusCode::OK,
            Json(LaunchResponse {
                status: "success".to_string(),
                message: "Agent successfully processed prompt".to_string(),
                agent_response: response,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(LaunchResponse {
                status: "error".to_string(),
                message: e.to_string(),
                agent_response: String::new(),
            }),
        ),
    }
}
