# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
just test          # cargo nextest run --all-features
just lint          # cargo clippy --all-targets --all-features -- -D warnings
just coverage      # llvm-cov HTML report (opens browser)
just up            # docker compose up -d (start Postgres)
just down          # docker compose down
just run-migration # sqlx migrate run
just serve         # bacon watch + run
```

Run a single test:

```sh
cargo nextest run <test_name>
```

## Setup

```sh
cp .env.example .env   # fill in OLLAMA_URL and ANTHROPIC_API_KEY
just up
just run-migration
cargo build
```

Required tools: `sqlx-cli` (`cargo install sqlx-cli`), `cargo-nextest` (`cargo install cargo-nextest`), Docker, `just`.

## Architecture

**Data flow:** user runs `sage index <type>` → indexer chunks the input → `embedder` calls Ollama `/api/embed` → `db::insert_chunk` stores chunk + 768-dim vector in Postgres (`embeddings` table). Then `sage chat` → TUI → agent → Claude API (SSE streaming) → tools query pgvector → TUI renders via `mpsc` channel.

**Key types:**

- `db::Chunk` — the unit stored in Postgres. Deduped by `(source, content_hash)` unique index (SHA-256). The `source` field is a logical name (e.g. `"my-project"`) used to scope searches.
- `agent::UiEvent` — the channel message type between the agent task and the TUI render loop: `TokenDelta`, `ToolStart`, `ToolDone`, `TurnComplete`.
- `agent::Agent` — holds `PgPool`, `Embedder`, API key/model, and conversation `history: Vec<Message>`.
- `agent::tools::Tool` — enum over `SearchDocs`, `SearchCode`, `ReadFile`, `FetchUrl`; deserializes from Claude's tool-call JSON via `serde`.

**Indexers** (`src/indexer/`): each produces `Vec<Chunk>` from a path. HTML chunks by `<h2>`/`<h3>` boundaries (scraper), PDF by section heading regex (pdf-extract), text/Markdown by heading or paragraph, code by function boundary (tree-sitter; C/C++ and Rust auto-detected by extension).

**Agent loop** (`src/agent/mod.rs`): `Agent::run_turn` appends user input to `history`, calls Claude API with streaming SSE, dispatches tool calls via `execute_tool`, caps at 10 tool-call rounds.

**TUI** (`src/tui.rs`): ratatui two-pane layout. Top pane: conversation + live tool status. Bottom pane: input bar. Agent runs as a separate tokio task; results arrive via `mpsc::Sender<UiEvent>`. Exit: `Ctrl+C` / `Ctrl+D`.

**DB schema:** single `embeddings` table with HNSW index (`vector_cosine_ops`) on the `embedding` column (768 dimensions = `nomic-embed-text`).

## Implementation state

The project is scaffolded (M1 complete): all modules, types, and fn signatures exist but most bodies are `todo!()`. See `ROADMAP.md` for the milestone plan (M2 = db/embedder, M3 = HTML/PDF indexers, M4 = code indexer, M5 = agent core, M6 = streaming + TUI, M7 = polish).

## Environment variables

| Variable            | Description                                       |
| ------------------- | ------------------------------------------------- |
| `DATABASE_URL`      | Postgres connection string                        |
| `OLLAMA_URL`        | Ollama base URL (e.g. `http://192.168.1.x:11434`) |
| `ANTHROPIC_API_KEY` | Claude API key                                    |
| `CLAUDE_MODEL`      | Model ID (default: `claude-sonnet-4-6`)           |
