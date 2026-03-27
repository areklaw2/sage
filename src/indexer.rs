pub mod code;
pub mod html;
pub mod pdf;
pub mod text;

use crate::cli::IndexSource;
use crate::db;
use crate::embedder::Embedder;
use sqlx::PgPool;

pub async fn run(source: IndexSource, pool: &PgPool, embedder: &Embedder) -> anyhow::Result<()> {
    todo!()
}
