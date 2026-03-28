# Roadmap

Work through these milestones in order. Each milestone should leave the binary in a working state (no panics beyond stubs not yet reached).

## M1 — Scaffold :white_check_mark:

All modules, types, and fn signatures exist with `todo!()` bodies. `cargo build` + `cargo test --list` pass.

## M2 — Infrastructure :white_check_mark:

- `DbConfig` — url, max_connections, connection_timeout; passed to `init_pool`
- `hash_content` — SHA-256 hex; callers hash before calling `chunk_exists`
- `chunk_exists` / `insert_chunk` — deduped by `(source, content_hash)` with `ON CONFLICT DO NOTHING`
- `embedder::embed` — POST to Ollama `/api/embed` with `qwen3-embedding:0.6b`, returns `embeddings[0]`
- DB integration tests use `testcontainers` (real pgvector container, no mocks)
- `main.rs` reads `DATABASE_URL` / `OLLAMA_URL`, builds pool + embedder, dispatches `Index` / `Chat`

## M3 — Document indexers (HTML + PDF + text)

**HTML — `indexer/html.rs`**

`scraper` parses HTML into a typed tree and lets you select nodes with CSS selectors (`.select("h2, h3")`). The strategy: iterate all nodes in document order, track the "current section" heading, accumulate text content until the next heading, emit a chunk. Tables should be emitted as a single chunk regardless of length — select `table` nodes and capture their full `.text()` before the heading-boundary logic runs. Look up: `scraper::Html::parse_document`, `scraper::Selector`, `ElementRef::text()`.

```
index_html(source: &str, html: &str) -> Vec<HtmlChunk>
```

**PDF — `indexer/pdf.rs`**

`pdf-extract` calls pdftotext under the hood and returns a raw `String` of all text. You lose formatting, but heading detection still works via regex on the line content. Strategy: split text by `\n`, scan lines for heading patterns (numbered sections like `4.1 Title` or `A.2 Title`, or ALL-CAPS short lines), use heading matches as chunk boundaries. Prepend the heading line to each chunk's content. Look up: `pdf_extract::extract_text`, `regex::Regex::is_match`.

```
index_pdf(source_name: &str, path: &Path) -> anyhow::Result<Vec<PdfChunk>>
```

**Text/Markdown — `indexer/text.rs`**

`walkdir` recursively walks a directory (or you handle a single file path). For Markdown: split on lines starting with `#` — each heading starts a new chunk, heading text becomes `section`. For plain `.txt`: split on double newlines `\n\n`. Either way, `file` field = path relative to the walk root (use `path.strip_prefix(root)`). Look up: `walkdir::WalkDir`, `std::path::Path::strip_prefix`, `str::split`.

```
index_text(source_name: &str, path: &Path) -> anyhow::Result<Vec<TextChunk>>
```

**Orchestration — `indexer::run()`**

Match the `IndexSource` variant → call the parser → for each chunk, call `embedder.embed(&chunk.content)` → fill a `db::Chunk` → call `db::insert_chunk`. Log inserted vs skipped count (the `ON CONFLICT DO NOTHING` means insert count < total when re-indexing).

**Tests:** add fixture files under `tests/fixtures/` — a small HTML file with `<h2>` sections and a table, a `.md` file with headings, a `.txt` file with paragraphs.

**Milestone exits when:** `sage index html/pdf/text` ingests real chunks end-to-end.

## M4 — Code indexer

**tree-sitter — `indexer/code.rs`**

tree-sitter parses source into a concrete syntax tree (CST). Each language has a grammar crate (`tree-sitter-rust`, `tree-sitter-c`) that you pass to `Parser::set_language`. You then run a **query** — a Lisp-like pattern — to find nodes: `(function_item name: (identifier) @name)` for Rust, `(function_definition declarator: (function_declarator declarator: (identifier) @name))` for C. `QueryCursor::matches` returns an iterator of captures. For each function node: get `start_position().row` and `end_position().row` (0-indexed, add 1 for display), get the source text with `node.utf8_text(source_bytes)`. To attach comments: scan lines backward from `start_position().row - 1` while lines are `//`, `///`, or `/* */`. Look up: `tree_sitter::Parser`, `tree_sitter::Query`, `tree_sitter::QueryCursor::matches`, `Node::utf8_text`.

```
index_code_repo(source_name: &str, root: &Path) -> anyhow::Result<Vec<CodeChunk>>
```

**Tests:** add `tests/fixtures/example.rs` and `tests/fixtures/example.c` with 2–3 functions each, some with doc comments. Assert correct `fn_name`, `line_start`, `line_end`, and that comment content is included.

**Milestone exits when:** `sage index code` ingests function chunks from a real Rust or C project.

## M5 — Agent core (non-streaming)

**Tools — `agent/tools.rs`**

`execute_tool` dispatches on the `Tool` enum:

- `SearchDocs` / `SearchCode`: call `embedder.embed(&query)` to get a `Vec<f32>`, then run a raw sqlx query using pgvector's `<=>` cosine distance operator: `SELECT content FROM embeddings WHERE source = $1 ORDER BY embedding <=> $2 LIMIT 5`. If `source` is `None`, omit the WHERE clause. Return the top chunks joined by `\n---\n`. Look up: `pgvector::Vector`, sqlx `query_as!` with a custom type, the `<=>` operator in pgvector docs.
- `ReadFile`: `tokio::fs::read_to_string`. Security: resolve the path with `std::fs::canonicalize` and reject anything that doesn't start with `std::env::current_dir()` — prevents `../` traversal attacks.
- `FetchUrl`: `reqwest::get(url).await?.text().await?`. Cap at 50 KB: truncate the string if `len() > 51_200`.

**Agent loop — `agent/mod.rs`**

The Claude Messages API: POST the full conversation history each turn. Request body: `model`, `max_tokens`, `system` (system prompt string), `messages` (history), `tools` (array of tool definitions with JSON schema). Response includes `stop_reason` and a `content` array.

Flow for `run_turn`:

1. Push `{role: "user", content: user_input}` to history
2. POST to `https://api.anthropic.com/v1/messages` with headers `x-api-key`, `anthropic-version: 2023-06-01`, `content-type: application/json`
3. If `stop_reason == "tool_use"`: extract `content` blocks of type `"tool_use"`, send `UiEvent::ToolStart(name)`, call `execute_tool`, send `UiEvent::ToolDone`, push assistant message + `{role: "user", content: [tool_result blocks]}` to history, loop
4. If `stop_reason == "end_turn"`: extract text blocks, send `UiEvent::TokenDelta` per block, then `UiEvent::TurnComplete`
5. Cap at 10 iterations to prevent runaway loops

`main.rs` Chat (no TUI yet): `loop { print!("> "); read_line → run_turn → print UiEvents to stdout }`.

Look up: Anthropic "Tool use overview" docs, `reqwest::Client::post`, `serde_json::Value` for flexible JSON parsing.

**Milestone exits when:** `sage chat` responds to a question and uses at least one tool via a read_line loop.

## M6 — Streaming + TUI

**Streaming — `agent/mod.rs`**

Add `"stream": true` to the request body. The response is SSE (Server-Sent Events): lines prefixed `data: ` containing JSON event objects. Event types to handle: `content_block_delta` with `delta.type == "text_delta"` → forward `delta.text` as `UiEvent::TokenDelta`; `delta.type == "input_json_delta"` → accumulate tool input JSON; `message_delta` with `delta.stop_reason` → detect end of turn. Use `response.bytes_stream()` from reqwest + manual line parsing. Look up: Claude "Streaming messages" docs, `futures::StreamExt`, `bytes::Bytes`.

**TUI — `tui.rs`**

ratatui renders to an alternate terminal screen. The pattern: `enable_raw_mode()` + `EnterAlternateScreen` → draw loop → `disable_raw_mode()` + `LeaveAlternateScreen` on exit. `Terminal::draw` takes a closure receiving a `Frame`; call `frame.render_widget(widget, area)`. Split layout with `Layout::vertical([Constraint::Fill(1), Constraint::Length(3)])` — top pane for conversation, bottom for input.

App state: `Vec<DisplayMessage>` (conversation), `String` (input buffer), `String` (tool status). Top pane: `Paragraph` with wrapped text and scroll offset. Bottom pane: `Paragraph` showing `"> {input_buf}"`.

Event loop: `crossterm::event::poll(Duration::ZERO)` for non-blocking reads; `KeyCode::Enter` → send to agent task, clear buf; `KeyCode::Char(c)` → append; `KeyCode::Backspace` → pop; `Ctrl+C`/`Ctrl+D` → exit. Agent runs in `tokio::spawn`; `UiEvent`s arrive on `mpsc::Receiver` polled each frame with `try_recv`.

Always restore terminal — wrap the run loop in a function that calls cleanup on both normal return and panic (`std::panic::set_hook` or a `Drop` guard). Look up: ratatui "Hello World" example, `crossterm::terminal::enable_raw_mode`, `crossterm::execute!`.

**Milestone exits when:** `sage chat` opens a TUI, streams tokens live, and shows tool status lines.

## M7 — Polish

**History trimming:** Claude's context window is 200k tokens. Rough estimate: `chars / 4 ≈ tokens`. If estimated total exceeds 160k, drop the oldest user+assistant pair from `history` (system prompt is a separate field, so `history[0]` is always a user message — safe to trim from the front). Keep at least the last 4 pairs.

**TUI error handling:** add `UiEvent::Error(String)` variant. If the agent task returns `Err`, send it as this variant. Render it in red using `Style::default().fg(Color::Red)` in the status line.

**Indexing progress:** record `Instant::now()` before `indexer::run()`, print `Indexed {n} chunks in {elapsed:.1}s` after completion.
