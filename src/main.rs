#![allow(dead_code, unused_variables, unused_imports)]

mod agent;
mod cli;
mod db;
mod embedder;
mod indexer;
mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let _cli = cli::parse();
    todo!()
}
