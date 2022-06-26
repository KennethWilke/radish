use anyhow::{anyhow, Result};
pub use axum::extract::ws::WebSocket;
pub use axum::extract::ws::WebSocketUpgrade;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Operation {
    pub operation: String,
    pub parameters: HashMap<String, String>,
}

impl Operation {
    pub fn new(operation: impl AsRef<str>) -> Self {
        let operation = operation.as_ref().to_string();
        let parameters = HashMap::new();
        Operation {
            operation,
            parameters,
        }
    }

    pub fn set_parameter(&mut self, key: impl AsRef<str>, value: impl AsRef<str>) {
        let key = key.as_ref().to_string();
        let value = value.as_ref().to_string();
        self.parameters.insert(key, value);
    }
}

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

    pub async fn get_operation(&mut self) -> Result<Operation> {
        let message = self.get_string().await?;
        match serde_json::from_str(message.as_str()) {
            Ok(operation) => {
                println!("Received operation: {:#?}", operation);
                Ok(operation)
            }
            _ => return Err(anyhow!("NYI")),
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
            Err(_) => Err(anyhow!("Websocket client disconnected")),
        }
    }

    pub async fn send_operation(&mut self, operation: Operation) -> Result<()> {
        let message = serde_json::to_string(&operation).unwrap();
        println!("Sending operation: {}", message);
        self.send_string(message).await
    }
}
