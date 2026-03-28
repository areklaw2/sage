use std::time::Duration;

use pgvector::Vector;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, postgres::PgPoolOptions};

pub struct DbConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout: u64,
}

pub struct Chunk {
    pub source: String,
    pub file: Option<String>,
    pub section: Option<String>,
    pub fn_name: Option<String>,
    pub line_start: Option<i32>,
    pub line_end: Option<i32>,
    pub content: String,
    pub content_hash: String,
    pub embedding: Option<Vec<f32>>,
}

pub fn hash_content(content: &str) -> String {
    hex::encode(Sha256::digest(content.as_bytes()))
}

pub async fn init_pool(db_config: DbConfig) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(db_config.max_connections)
        .acquire_timeout(Duration::from_secs(db_config.connection_timeout))
        .connect(&db_config.url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> anyhow::Result<()> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn chunk_exists(pool: &PgPool, source: &str, hash: &str) -> anyhow::Result<bool> {
    let exists = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM embeddings WHERE source = $1 AND content_hash = $2)",
    )
    .bind(source)
    .bind(hash)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}

pub async fn insert_chunk(pool: &PgPool, chunk: &Chunk) -> anyhow::Result<()> {
    let hash = hash_content(&chunk.content);
    let embedding = chunk.embedding.as_ref().map(|v| Vector::from(v.clone()));

    sqlx::query(
    "INSERT INTO embeddings (source, file, section, fn_name, line_start, line_end, content, content_hash, embedding)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        ON CONFLICT (source, content_hash) DO NOTHING"
    )
    .bind(&chunk.source)
    .bind(&chunk.file)
    .bind(&chunk.section)
    .bind(&chunk.fn_name)
    .bind(chunk.line_start)
    .bind(chunk.line_end)
    .bind(&chunk.content)
    .bind(&hash)
    .bind(embedding)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_content_is_sha256_hex() {
        // SHA-256 of empty string is known
        assert_eq!(
            hash_content(""),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_chunk_struct_fields() {
        let chunk = Chunk {
            source: "html".to_string(),
            file: None,
            section: Some("Section".to_string()),
            fn_name: None,
            line_start: None,
            line_end: None,
            content: "This section explains".to_string(),
            content_hash: "abc123".to_string(),
            embedding: None,
        };
        assert_eq!(chunk.source, "html");
        assert_eq!(chunk.section.as_deref(), Some("Section"));
        assert_eq!(chunk.content_hash, "abc123");
    }

    use testcontainers::{
        GenericImage, ImageExt,
        core::{IntoContainerPort, WaitFor},
        runners::AsyncRunner,
    };

    async fn start_db() -> (PgPool, impl std::any::Any) {
        let container = GenericImage::new("pgvector/pgvector", "pg18")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_DB", "sage")
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "postgres")
            .start()
            .await
            .unwrap();

        let port = container.get_host_port_ipv4(5432).await.unwrap();
        let pool = init_pool(DbConfig {
            url: format!("postgres://postgres:postgres@127.0.0.1:{}/sage", port),
            max_connections: 2,
            connection_timeout: 10,
        })
        .await
        .unwrap();
        run_migrations(&pool).await.unwrap();

        (pool, container)
    }

    #[tokio::test]
    async fn test_insert_chunk_and_exists() {
        let (pool, _container) = start_db().await;

        let content = "integration test content unique-1a2b3c";
        let hash = hash_content(content);
        let chunk = Chunk {
            source: "test".to_string(),
            file: None,
            section: None,
            fn_name: None,
            line_start: None,
            line_end: None,
            content: content.to_string(),
            content_hash: hash.clone(),
            embedding: None,
        };

        insert_chunk(&pool, &chunk).await.unwrap();
        assert!(chunk_exists(&pool, "test", &hash).await.unwrap());

        // second insert is a no-op — no error, no duplicate
        insert_chunk(&pool, &chunk).await.unwrap();
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM embeddings WHERE source = $1 AND content_hash = $2",
        )
        .bind("test")
        .bind(&hash)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(count.0, 1);
    }

    #[tokio::test]
    async fn test_chunk_exists_returns_false_for_unknown() {
        let (pool, _container) = start_db().await;
        assert!(
            !chunk_exists(&pool, "test", "nonexistent-hash")
                .await
                .unwrap()
        );
    }
}
