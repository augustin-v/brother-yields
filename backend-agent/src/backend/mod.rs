use crate::agents::navigator::{launch, Navigator, Tools};
use crate::types::{ProtocolYield, Token};
use axum::extract::State;
use axum::{
    http::{header, Method, StatusCode},
    routing::{get, post},
    Json, Router,
};
use parking_lot::RwLock;
use rig::agent::AgentBuilder;
use rig::completion::{CompletionModel, Message};
use tokio::sync::mpsc::Sender;
use tower_http::cors::CorsLayer;
use crate::agents::AgentState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tokio::sync::{Mutex, mpsc};
use tracing::info;
use url::Url;
use messaging::{ChatHistoryCommand, ChatHistoryManager, spawn_chat_history_manager};

pub mod messaging;

#[derive(Clone)]
pub struct Backend<M: CompletionModel> {
    pub is_active: Arc<AtomicBool>,
    pub listener_addr: Arc<RwLock<Option<Url>>>,
    pub app_state: Arc<Mutex<AppState<M>>>,
    pub yields_data: Vec<ProtocolYield>,
}

#[derive(Clone)]
pub struct AppState<M: CompletionModel> {
    pub agent_state: Option<AgentState<M>>,
    pub portfolio_data: Arc<RwLock<HashMap<String, HashMap<Token, f64>>>>,
    pub chat_sender: mpsc::Sender<ChatHistoryCommand>,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    status: String,
    message: String,
}

#[derive(Deserialize)]
pub struct PromptRequest {
    prompt: String,
    session_id: String
}

#[derive(Deserialize)]
pub struct ValidateSessionRequest {
    session_id: String,
}

#[derive(Serialize)]
pub struct YieldsResponse {
    yields: Vec<ProtocolYield>,
}

impl<M: CompletionModel + 'static> Backend<M> {
    pub fn new(yields_data: Vec<ProtocolYield>, manager: ChatHistoryManager) -> Self {
        
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            listener_addr: Arc::new(RwLock::new(None)),
            app_state: Arc::new(Mutex::new(AppState::new(manager.get_sender()))),
            yields_data,
        }
    }

    pub async fn start(
        self,
        nav_model: M,
        defaigent_model: AgentBuilder<M>,
        tools: Tools<M>,
        receiver: mpsc::Receiver<ChatHistoryCommand>,
    ) -> Result<(), anyhow::Error> {
        let sessions = Arc::new(Mutex::new(HashMap::<String, Vec<Message>>::new()));
        spawn_chat_history_manager(receiver, sessions.clone());
        info!("getting sender...");
        let chat_sender = {self.app_state.lock().await.chat_sender.clone()};
        info!("got sender");
        self.app_state.lock().await.agent_state = Some(AgentState {
            navigator: Arc::new(Mutex::new(Navigator::new(
                nav_model,
                defaigent_model,
                tools,
                chat_sender,
            ))),
        });

        let cors = CorsLayer::new()
            .allow_origin(
                "https://brother-yields.vercel.app"
                    .parse::<header::HeaderValue>()
                    .unwrap(),
            )
            .allow_methods([Method::GET, Method::POST])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_credentials(true);

        let app = Router::new()
            .route("/init-session", get(init_session_handler))
            .route("/validate-session", post(validate_session_handler)) // Add this line
            .route("/launch", post(launch_handler))
            .route("/prompt", post(prompt_handler))
            .route("/yields", get(yields_handler))
            .layer(cors)
            .with_state(self.clone());

        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000")
            .await
            .expect("Listener failure");
        info!("Listener started on 0.0.0.0:8000");

        self.is_active.store(true, Ordering::SeqCst);
        *self.listener_addr.write() = Some(Url::parse("http://0.0.0.0:8000/").unwrap());

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

impl<M: CompletionModel> AppState<M> {
    pub fn new(chat_sender: mpsc::Sender<ChatHistoryCommand>) -> Self {
        Self {
            agent_state: None,
            portfolio_data: Arc::new(RwLock::new(HashMap::new())),
            chat_sender,
        }
    }
    pub fn update_portfolio(&self, address: String, balances: HashMap<Token, f64>) {
        let mut data = self.portfolio_data.write();
        data.insert(address.clone(), balances);
        info!("User {} portfolio updated", address);
    }
}

/// Testing function
pub async fn launch_handler<M: CompletionModel +'static>(
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

pub async fn prompt_handler<M: CompletionModel + 'static>(
    State(backend): State<Backend<M>>,
    Json(request): Json<PromptRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    info!("received call");
    
    let nav_agent = backend
        .app_state
        .lock()
        .await
        .agent_state
        .clone()
        .expect("No agent available")
        .navigator;

    info!("appstate locked nav locked");
    let byebye = match nav_agent.lock().await.process_prompt(&request.prompt, request.session_id).await {
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


pub async fn init_session_handler<M: CompletionModel + 'static>(
    State(backend): State<Backend<M>>,
) -> (StatusCode, Json<ApiResponse>) {
    info!("init session");
    let session_id = uuid::Uuid::new_v4().to_string();
    
    let chat_sender  = {
        let state = backend.app_state.lock().await;
            state.chat_sender.clone()
    };

    // Create new session
    chat_sender.send(ChatHistoryCommand::CreateSession(session_id.clone()))
        .await
        .expect("Failed creating session");
    
    // Add confirmation channel
    let (tx, rx) = tokio::sync::oneshot::channel();
    chat_sender.send(ChatHistoryCommand::GetHistory(session_id.clone(), tx))
        .await
        .expect("Failed to verify session");
    
    // Wait for session to be available
    rx.await.expect("Failed to confirm session creation");
    
    info!("created session: {}", session_id);
    (StatusCode::OK, Json(ApiResponse {
        status: "success".to_string(),
        message: session_id,
    }))
}

pub async fn validate_session_handler<M: CompletionModel + 'static>(
    State(backend): State<Backend<M>>,
    Json(request): Json<ValidateSessionRequest>,
) -> (StatusCode, Json<ApiResponse>) {
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    let chat_sender = {
        let state = backend.app_state.lock().await;
        state.chat_sender.clone()
    };

    match chat_sender.send(ChatHistoryCommand::GetHistory(request.session_id.clone(), tx)).await {
        Ok(_) => {
            match rx.await {
                Ok(history) if !history.is_empty() => (
                    StatusCode::OK,
                    Json(ApiResponse {
                        status: "success".to_string(),
                        message: "Session is valid".to_string(),
                    })
                ),
                _ => (
                    StatusCode::NOT_FOUND,
                    Json(ApiResponse {
                        status: "error".to_string(),
                        message: "Session not found or invalid".to_string(),
                    })
                )
            }
        },
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                status: "error".to_string(),
                message: "Failed to validate session".to_string(),
            })
        )
    }
}
