use crate::agent::Agent;

struct DisplayMessage {
    role: String,
    text: String,
}

struct App {
    input_buf: String,
    messages: Vec<DisplayMessage>,
    status: Option<String>,
}

/// Run the ratatui TUI. Two-pane layout:
/// - Top: conversation history + live tool call status lines
/// - Bottom: single-line input bar
/// Ctrl+C / Ctrl+D exit cleanly and restore the terminal.
pub async fn run(agent: Agent) -> anyhow::Result<()> {
    todo!()
}
