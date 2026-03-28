export DATABASE_URL := "postgres://postgres:postgres@localhost:5432/sage"

# Set up local dev database: start container + run migrations
db: up run-migration

# Start the Postgres container
up:
  docker compose up -d --wait

# Stop the Postgres container
down:
  docker compose down

# Run the CLI
serve:
  bacon run-long

# Run tests
test:
  cargo nextest run --all-features

# Run tests with coverage
coverage:
  cargo llvm-cov --all-features --workspace --html && open target/llvm-cov/html/index.html

# Lint with clippy
lint:
  cargo clippy --all-targets --all-features -- -D warnings

# Create the local database
create-db:
  sqlx database create

# Create a new migration file (e.g. just create-migration add-table)
create-migration name:
  sqlx migrate add {{name}}

# Apply pending migrations
run-migration:
  sqlx migrate run
