use rig::completion::Message;
use tokio::sync::{mpsc, oneshot, Mutex};
use tracing::info;
use std::collections::HashMap;
use std::sync::Arc;
pub struct ChatHistoryManager {
    pub sessions: HashMap<String, Vec<Message>>,
    pub sender: mpsc::Sender<ChatHistoryCommand>,
    pub message_limit: usize
}


pub enum ChatHistoryCommand {
    AddMessage(String, Message),
    GetHistory(String, oneshot::Sender<Vec<Message>>),
    CreateSession(String),
    DeleteSession(String),
}

impl ChatHistoryManager {
    pub fn new() -> (Self, mpsc::Receiver<ChatHistoryCommand>) {
        let (sender, receiver) = mpsc::channel(100);
        (Self {
            sessions: HashMap::new(),
            sender,
            message_limit: 5,
        }, receiver)
    }

    pub fn get_sender(&self) -> mpsc::Sender<ChatHistoryCommand> {
        self.sender.clone()
    }
}

pub fn spawn_chat_history_manager(mut receiver: mpsc::Receiver<ChatHistoryCommand>, sessions: Arc<Mutex<HashMap<String, Vec<Message>>>>) {
    let message_limit = 5;

    tokio::spawn(async move {
        info!("Chat history manager started");
        while let Some(cmd) = receiver.recv().await {
            let mut sessions_lock = sessions.lock().await;
            match cmd {
                ChatHistoryCommand::AddMessage(session_id, msg) => {
                    info!("Adding message to session {}", session_id);
                    info!("Current sessions: {:?}", sessions_lock.keys().collect::<Vec<_>>());
                    if let Some(history) = sessions_lock.get_mut(&session_id) {
                        history.push(msg);
                        info!("Msg added to session: {}, history length: {}", session_id, history.len());
                        if history.len() > message_limit {
                            history.remove(0);
                        }
                    } else {
                        tracing::warn!("Attempted to add message to non-existent session: {}", session_id);
                    }
                }
                ChatHistoryCommand::GetHistory(session_id, respond_to) => {
                    let history = sessions_lock
                        .get(&session_id)
                        .map(|h| h.clone())
                        .unwrap_or_default();
                    let _ = respond_to.send(history);
                }
                ChatHistoryCommand::CreateSession(session_id) => {
                    sessions_lock.insert(session_id.clone(), Vec::new());
                }
                ChatHistoryCommand::DeleteSession(session_id) => {
                    sessions_lock.remove(&session_id);
                }
            }
        }
    });
}
