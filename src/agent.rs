pub mod prompt;
pub mod tools;

use crate::embedder::Embedder;
use serde_json::Value;
use sqlx::PgPool;
use tokio::sync::mpsc;

pub enum UiEvent {
    TokenDelta(String),
    ToolStart { name: String, input: String },
    ToolDone { result: String },
    TurnComplete,
}

pub struct Message {
    pub role: String,
    pub content: Value,
}

pub struct Agent {
    pub pool: PgPool,
    pub embedder: Embedder,
    pub api_key: String,
    pub model: String,
    pub history: Vec<Message>,
}

impl Agent {
    pub fn new(pool: PgPool, embedder: Embedder, api_key: String, model: String) -> Self {
        Self {
            pool,
            embedder,
            api_key,
            model,
            history: Vec::new(),
        }
    }

    /// Run one turn of the agentic loop. Streams tokens and tool calls via `ui_tx`.
    /// Caps at 10 tool-call rounds before returning an error.
    pub async fn run_turn(
        &mut self,
        user_input: &str,
        ui_tx: mpsc::Sender<UiEvent>,
    ) -> anyhow::Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ui_event_variants_exist() {
        let _ = UiEvent::TokenDelta("hello".to_string());
        let _ = UiEvent::ToolStart {
            name: "search_docs".to_string(),
            input: "Speed of light".to_string(),
        };
        let _ = UiEvent::ToolDone {
            result: "The speed of light".to_string(),
        };
        let _ = UiEvent::TurnComplete;
    }
}
