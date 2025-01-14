use crate::agents::navigator::{launch, Navigator, Tools};
use crate::types::ProtocolYield;
use axum::extract::State;
use axum::{
    http::{StatusCode, Method, header},
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use rig::completion::CompletionModel;
use tower_http::cors::CorsLayer;

use crate::agents::AgentState;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::Mutex;
use tracing::info;
use url::Url;

#[derive(Clone)]
pub struct Backend<M: CompletionModel> {
    pub is_active: Arc<AtomicBool>,
    pub listener_addr: Arc<RwLock<Option<Url>>>,
    pub agent_state: Option<AgentState<M>>,
    pub yields_data: Vec<ProtocolYield>,
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

#[derive(Serialize)]
pub struct YieldsResponse {
    yields: Vec<ProtocolYield>,
}

impl<M: CompletionModel + 'static> Backend<M> {
    pub fn new(yields_data: Vec<ProtocolYield>) -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            listener_addr: Arc::new(RwLock::new(None)),
            agent_state: None,
            yields_data,
        }
    }

    pub async fn start(mut self, model: M, tools: Tools, context: String) -> Result<(), anyhow::Error> {
        self.agent_state = Some(AgentState {
            navigator: Arc::new(Mutex::new(Navigator::new(model, tools, context))),
        });

        let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<header::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true);

        let app = Router::new()
            .route("/launch", post(launch_handler))
            .route("/prompt", post(prompt_handler))
            .route("/yields", get(yields_handler))
            .layer(cors)
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
pub async fn launch_handler<M: CompletionModel>(
    State(backend): State<Backend<M>>,
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

pub async fn prompt_handler<M: CompletionModel>(
    State(backend): State<Backend<M>>,
    Json(request): Json<PromptRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    let nav_agent = backend
        .agent_state
        .clone()
        .expect("No agent available")
        .navigator;

    let byebye = match nav_agent.lock().await.process_prompt(&request.prompt).await {
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
    };
    byebye
}

pub async fn yields_handler<M: CompletionModel>(
    State(backend): State<Backend<M>>,
) -> (StatusCode, Json<YieldsResponse>) {
    (
        StatusCode::OK,
        Json(YieldsResponse {
            yields: backend.yields_data,
        }),
    )
}
