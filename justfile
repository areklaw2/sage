export DATABASE_URL := "postgres://postgres:postgres@localhost:5432/sage"

# Start containers, pull models, and run migrations
setup: start pull-models migrate

# Start all containers (Postgres + Ollama)
start:
  docker compose up -d --wait

# Stop all containers
stop:
  docker compose down

# Tear down containers and wipe all volumes (destructive — deletes all indexed data)
reset:
  docker compose down -v

# Pull required Ollama embedding models
pull-models:
  docker compose exec ollama ollama pull qwen3-embedding:0.6b

# Run pending database migrations
migrate:
  sqlx migrate run

# Create a new migration file (e.g. just new-migration add-table)
new-migration name:
  sqlx migrate add {{name}}

# Run the CLI with live reload
serve:
  bacon run-long

# Run all tests
test:
  cargo nextest run --all-features

# Run tests with HTML coverage report
coverage:
  cargo llvm-cov --all-features --workspace --html && open target/llvm-cov/html/index.html

# Lint with clippy
lint:
  cargo clippy --all-targets --all-features -- -D warnings
