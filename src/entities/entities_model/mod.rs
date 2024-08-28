use axum::response::{IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;



#[derive(Deserialize)]
pub struct PushRequest {
    pub message: String,
}


#[derive(Serialize)]
pub enum QueueResponder {
    Push {
        message: String,
        id: u8
    },
    Pop {
        message: String,
        metadata: String
    }
}

impl QueueResponder {
    pub fn respond_json(self) -> impl IntoResponse {
        match self {
            QueueResponder::Push { message, id } => {
                let mut push_response: HashMap<String, String> = HashMap::new();
                push_response.insert("Operation".to_string(), "push".to_string());
                push_response.insert("Value".to_string(), message.clone());
                push_response.insert("Message ID:".to_string(), format!("{}",id));
                Json(push_response)
            }
            QueueResponder::Pop { message, metadata } => {
                let mut pop_response: HashMap<String, String> = HashMap::new();
                pop_response.insert("Operation".to_string(), "pop".to_string());
                pop_response.insert("Value".to_string(), message.clone());
                pop_response.insert("Metadata".to_string(), metadata.clone());
                Json(pop_response)
            }
        }
    }
}