pub mod code;
pub mod html;
pub mod pdf;
pub mod text;

use crate::cli::IndexSource;
use crate::db;
use crate::embedder::Embedder;
use crate::indexer::html::index_html;
use sqlx::PgPool;
use tokio::fs;

pub async fn run(source: IndexSource, pool: &PgPool, embedder: &Embedder) -> anyhow::Result<()> {
    match source {
        IndexSource::Html { source, path } => {
            let source = source.unwrap_or_else(|| {
                path.file_stem()
                    .expect("path has no filename")
                    .to_string_lossy()
                    .into_owned()
            });
            let html = fs::read_to_string(path).await?;
            let chunks = index_html(&source, html.as_str());
        }
        IndexSource::Pdf { source, path } => todo!(),
        IndexSource::Text { source, path } => todo!(),
        IndexSource::Code { source, path } => todo!(),
    }

    Ok(())
}
