# Roadmap

Work through these milestones in order. Each milestone should leave the binary in a working state (no panics beyond stubs not yet reached).

## M1 — Scaffold :white_check_mark:

All module files, types, fn signatures, `todo!()` bodies, test stubs. `cargo build` + `cargo test --list` passes.

## M2 — Infrastructure

- Implement `db::init_pool`, `run_migrations`, `chunk_exists`, `insert_chunk`
- Implement `embedder::embed` (Ollama `/api/embed` call)
- `docker compose up` → migrations run → embed a test string → row inserted

## M3 — Document indexers (HTML + PDF)

- Implement `indexer/html.rs`: scraper-based section chunker for HTML documentation
- Implement `indexer/pdf.rs`: pdf-extract text extraction, regex-based section heading detection, chunk by section boundary
- Unit tests pass on fixture HTML snippets and a single-page PDF fixture
- `sage index html --path ./docs/spec.htm` and `sage index pdf --source lang --path ./docs/lang.pdf` both ingest real chunks

## M4 — Code indexer

- Implement `indexer/code.rs`: tree-sitter-c function extractor
- Unit tests pass on fixture C snippets
- `sage index code --source my-project --path ./repos/my-project/src` ingests functions

## M5 — Agent core (non-streaming)

- Implement `agent/tools.rs`: all four tools
- Implement `agent/mod.rs` agentic loop — non-streaming first (simpler), 10-turn cap
- `sage chat` dispatches to a plain `read_line` loop temporarily; agent responds

## M6 — Streaming + TUI

- Swap non-streaming for SSE streaming in `agent/mod.rs`
- Implement `tui.rs`: ratatui two-pane layout, crossterm event loop, `mpsc` channel to agent task
- `sage chat` opens TUI, streams tokens live, shows tool status lines

## M7 — Polish

- Conversation history trimming if needed
- Error messages in TUI instead of panics
