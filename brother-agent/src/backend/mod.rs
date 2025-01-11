use crate::agents::{navigator::launch, BrotherAgent};
use axum::{http::StatusCode, routing::post, Json, Router};
use axum::extract::State;
use axum;
use parking_lot::RwLock;
use serde::{Serialize, Deserialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use tracing::info;
use url::Url;
use crate::agents::AgentState;
use crate::agents::navigator;

#[derive(Clone)]
pub struct Backend {
    pub is_active: Arc<AtomicBool>,
    pub listener_addr: Arc<RwLock<Option<Url>>>,
    pub agent_state: Option<AgentState>,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    status: String,
    message: String,
}

#[derive(Deserialize)]
pub struct PromptRequest {
    prompt: String,
}


impl Backend {
    pub fn new() -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            listener_addr: Arc::new(RwLock::new(None)),
            agent_state: None
        }
    }

    pub async fn start(mut self, api_key: String) -> Result<(), anyhow::Error> {
        self.agent_state = Some(AgentState{
            navigator:  Arc::new(Mutex::new(BrotherAgent::from(navigator::agent_build(api_key).await.expect("Failed building agent"), crate::agents::AgentRole::Navigator))),
        });
        let app = Router::new().route("/launch", post(launch_handler))
                                        .route("/prompt", post(prompt_handler))
                                        .with_state(self.clone());

        let listener = tokio::net::TcpListener::bind("127.0.0.1:5050")
            .await
            .expect("Listener failure");
        info!("Listener started on 127.0.0.1:300");

        self.is_active.store(true, Ordering::SeqCst);
        *self.listener_addr.write() = Some(Url::parse("http://127.0.0.1:5050/").unwrap());

        info!(
            "Backend is active: {} {}",
            self.is_active.load(Ordering::SeqCst),
            self.clone().listener_addr.read().as_ref().unwrap()
        );
        axum::serve(listener, app)
            .await
            .expect("axum serving failure");
        Ok(())
    }

}

/// Testing function
#[axum::debug_handler]
pub async fn launch_handler(
    State(backend): State<Backend>
) -> (StatusCode, Json<ApiResponse>) {
    match launch(&backend).await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse {
                status: "success".to_string(),
                message: "Agent successfully launched".to_string(),
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                status: "error".to_string(),
                message: e.to_string(),
            }),
        ),
    }
}

#[axum::debug_handler]
pub async fn prompt_handler(
    State(backend): State<Backend>,
    Json(request): Json<PromptRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    let nav_agent = backend
        .agent_state
        .clone()
        .expect("No agent available")
        .navigator;
    
    let byebye = match nav_agent.lock().await.proccess_message(&request.prompt).await {
        Ok(response) => (
            StatusCode::OK,
            Json(ApiResponse {
                status: "success".to_string(),
                message: response,
            }),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                status: "error".to_string(),
                message: e.to_string(),
            }),
        ),
    }; byebye
}