use serde::{Deserialize, Serialize};

pub struct Embedder {
    pub client: reqwest::Client,
    pub base_url: String,
    pub model: String,
}

#[derive(Serialize)]
struct EmbedRequest<'a> {
    model: &'a str,
    input: &'a str,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embeddings: Vec<Vec<f32>>,
}

impl Embedder {
    pub fn new(base_url: String, model: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            base_url,
            model,
        }
    }

    pub async fn embed(&self, text: &str) -> anyhow::Result<Vec<f32>> {
        let url = format!("{}/api/embed", self.base_url);
        let resp: EmbedResponse = self
            .client
            .post(&url)
            .json(&EmbedRequest {
                model: &self.model,
                input: text,
            })
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;

        resp.embeddings
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::anyhow!("empty embeddings response from Ollama"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedder_new_stores_fields() {
        let e = Embedder::new(
            "http://192.168.1.1:11434".to_string(),
            "nomic-embed-text".to_string(),
        );
        assert_eq!(e.base_url, "http://192.168.1.1:11434");
        assert_eq!(e.model, "nomic-embed-text");
    }

    #[tokio::test]
    async fn test_embed_bad_url_returns_error() {
        let e = Embedder::new(
            "http://127.0.0.1:1".to_string(),
            "nomic-embed-text".to_string(),
        );
        assert!(e.embed("hello").await.is_err());
    }
}
