use anyhow::{anyhow, Result};
pub use axum::extract::ws::WebSocket;
pub use axum::extract::ws::WebSocketUpgrade;

mod operation;

pub use operation::{Operation, OperationReply};

pub struct RadishWebSocket {
    pub socket: WebSocket,
}

impl RadishWebSocket {
    pub fn new(socket: WebSocket) -> Self {
        RadishWebSocket { socket }
    }

    pub async fn get_string(&mut self) -> Result<String> {
        match self.socket.recv().await {
            Some(result) => match result {
                Ok(message) => match message.to_text() {
                    Ok(message) => Ok(message.to_string()),
                    _ => Err(anyhow!("Message failed to convert to string")),
                },
                _ => Err(anyhow!("Message read result was an error")),
            },
            _ => Err(anyhow!("Socket recv failed")),
        }
    }

    pub async fn send_string(&mut self, message: impl AsRef<str>) -> Result<()> {
        let message = message.as_ref().to_string();
        match self
            .socket
            .send(axum::extract::ws::Message::Text(message))
            .await
        {
            Ok(_) => Ok(()),
            Err(error) => Err(anyhow!(error)),
        }
    }

    pub async fn get_operation(&mut self) -> Result<Operation> {
        let message = self.get_string().await?;
        match serde_json::from_str(message.as_str()) {
            Ok(operation) => Ok(operation),
            Err(error) => return Err(anyhow!(error)),
        }
    }

    pub async fn send_operation(&mut self, operation: Operation) -> Result<()> {
        let message = serde_json::to_string(&operation).unwrap();
        self.send_string(message).await
    }

    pub async fn get_reply(&mut self) -> Result<OperationReply> {
        let message = self.get_string().await?;
        match serde_json::from_str(message.as_str()) {
            Ok(reply) => Ok(reply),
            Err(error) => return Err(anyhow!(error)),
        }
    }

    pub async fn send_reply(&mut self, reply: OperationReply) -> Result<()> {
        let message = serde_json::to_string(&reply).unwrap();
        self.send_string(message).await
    }
}
