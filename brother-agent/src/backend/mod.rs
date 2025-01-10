use axum::{routing::post, Router, Json, http::StatusCode};
use serde::{Deserialize,Serialize};
use crate::agents::navigator::launch;
use tracing::info;
use url::Url;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use parking_lot::RwLock;

#[derive(Debug, Clone)]
pub struct Backend {
    pub is_active: Arc<AtomicBool>,
    pub listener_addr: Arc<RwLock<Option<Url>>>
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    status: String,
    message: String
}

impl Backend {
    pub fn new() -> Self {
        Self {
            is_active: Arc::new(AtomicBool::new(false)),
            listener_addr: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn start(mut self) -> Result<(), anyhow::Error> {
        let app = Router::new()
        .route("/launch", post(launch_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5050")
        .await
        .expect("Listener failure");
    info!("Listener started on 127.0.0.1:300");

    self.is_active.store(true, Ordering::SeqCst);
    *self.listener_addr.write() = Some(Url::parse("http://127.0.0.1:5050/").unwrap());
    axum::serve(listener, app).await.expect("axum serving failure");
    Ok(())
    }
}

async fn launch_handler() -> (StatusCode, Json<ApiResponse>) {
    match launch().await {
        Ok(_) => (
            StatusCode::OK,
            Json(ApiResponse {
                status: "success".to_string(),
                message: "Agent successfully launched".to_string()
            })
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse {
                status: "error".to_string(),
                message: e.to_string(),
            })
        )
    }
}
