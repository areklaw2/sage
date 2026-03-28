#![allow(dead_code, unused_variables, unused_imports)]

use crate::{cli::Commands, db::DbConfig, embedder::Embedder};

mod agent;
mod cli;
mod db;
mod embedder;
mod indexer;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let cli = cli::parse();

    let db_config = DbConfig {
        url: std::env::var("DATABASE_URL")?,
        max_connections: 5,
        connection_timeout: 5,
    };

    let pool = db::init_pool(db_config).await?;
    db::run_migrations(&pool).await?;

    let embedder = Embedder::new(
        std::env::var("OLLAMA_URL")?,
        "qwen3-embedding:0.6b".to_string(),
    );

    match cli.command {
        Commands::Index { source } => indexer::run(source, &pool, &embedder).await?,
        Commands::Chat => todo!("Implement Chat"),
    }

    Ok(())
}
