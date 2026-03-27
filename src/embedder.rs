pub struct Embedder {
    pub client: reqwest::Client,
    pub base_url: String,
    pub model: String,
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
        todo!()
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
}
