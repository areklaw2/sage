# sage

A local AI assistant. Indexes HTML, PDFs, plain text, and source code into pgvector, then provides an agentic chat loop that can search and reason over that corpus. Reasoning via the Claude API. Embeddings via Ollama running on a local Pi 5.

## Prerequisites

- Rust (edition 2024)
- Docker (for Postgres + pgvector)
- [just](https://github.com/casey/just)
- [sqlx-cli](https://github.com/launchbadge/sqlx): `cargo install sqlx-cli`
- [cargo-nextest](https://nexte.st): `cargo install cargo-nextest`
- Ollama running on your Pi 5 with `nomic-embed-text` pulled

## Setup

```sh
cp .env.example .env
# fill in OLLAMA_URL and ANTHROPIC_API_KEY

just up           # start Postgres
just run-migration  # apply schema
cargo build
```

## Usage

**Index documentation:**

```sh
sage index html --path ./doc.html
sage index pdf  --path ./doc.pdf
sage index text --path ./doc.md/ # plain text or Markdown files
```

**Index source code:**

```sh
sage index code --source repo --path ./repos/some-repo  # C/C++ and Rust, auto-detected by extension
```

Re-running any index command skips chunks that are already indexed (deduped by SHA-256 content hash).

**Chat:**

```sh
sage chat
```

Opens a two-pane TUI. Top pane shows conversation history and live tool call status. Bottom pane is the input bar. `Ctrl+D` or `Ctrl+C` to exit.

## Development

```sh
just test      # run tests with nextest
just lint      # clippy -D warnings
just coverage  # llvm-cov HTML report
just serve     # bacon watch + run
```

## Architecture

```
src/
├── cli.rs           — clap subcommands
├── db.rs            — sqlx pool, chunk insert/dedup helpers
├── embedder.rs      — Ollama /api/embed client
├── tui.rs           — ratatui two-pane chat interface
├── indexer/
│   ├── html.rs      — scraper, chunks by <h2>/<h3> boundary
│   ├── pdf.rs       — pdf-extract, chunks by section heading
│   ├── text.rs      — plain text / Markdown, chunks by heading or paragraph
│   └── code.rs      — tree-sitter (C/C++, Rust), chunks by function boundary
└── agent/
    ├── mod.rs       — Claude API agentic loop, SSE streaming, 10-turn cap
    ├── tools.rs     — search_docs, search_code, read_file, fetch_url
    └── prompt.rs    — system prompt
```

The agent communicates with the TUI via an `mpsc` channel of `UiEvent`s (token deltas, tool start/done, turn complete), keeping the render loop responsive during streaming.

## Environment variables

| Variable            | Description                                       |
| ------------------- | ------------------------------------------------- |
| `DATABASE_URL`      | Postgres connection string                        |
| `OLLAMA_URL`        | Ollama base URL (e.g. `http://192.168.1.x:11434`) |
| `ANTHROPIC_API_KEY` | Claude API key                                    |
| `CLAUDE_MODEL`      | Model ID (default: `claude-sonnet-4-6`)           |

## Roadmap

See [ROADMAP.md](ROADMAP.md).
