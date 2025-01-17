use rig::completion::Message;
use tokio::sync::{mpsc, oneshot};
use tracing::info;
pub struct ChatHistoryManager {
    pub history: Vec<Message>,
    pub sender: mpsc::Sender<ChatHistoryCommand>,
}

pub enum ChatHistoryCommand {
    AddMessage(Message),
    GetHistory(oneshot::Sender<Vec<Message>>),
}

impl ChatHistoryManager {
    pub fn new() -> (Self, mpsc::Receiver<ChatHistoryCommand>) {
        let (sender, receiver) = mpsc::channel(100);
        (Self {
            history: Vec::new(),
            sender,
        }, receiver)
    }

    pub fn get_sender(&self) -> mpsc::Sender<ChatHistoryCommand> {
        self.sender.clone()
    }
}

pub fn spawn_chat_history_manager(mut receiver: mpsc::Receiver<ChatHistoryCommand>) {
    tokio::spawn(async move {
        let mut history = Vec::new();
        info!("Chat history manager started");
        while let Some(cmd) = receiver.recv().await {
            match cmd {
                ChatHistoryCommand::AddMessage(msg) => {
                    info!("Adding message to history: {:?}", msg);
                    history.push(msg);
                    info!("Current history length: {}", history.len());
                }
                ChatHistoryCommand::GetHistory(respond_to) => {
                    info!("Getting history, current length: {}", history.len());
                    let _ = respond_to.send(history.clone());
                }
            }
        }
    });
}
