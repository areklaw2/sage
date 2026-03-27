pub struct HtmlChunk {
    pub source: String,
    pub section: String,
    pub content: String,
}

/// Parse an HTML document and chunk it by <h2>/<h3> section boundaries.
/// Tables are never split — they attach to the preceding section.
/// The heading text is included at the start of each chunk.
pub fn parse_html(source: &str, html: &str) -> Vec<HtmlChunk> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_html_returns_chunks() {
        todo!()
    }

    #[test]
    fn test_parse_html_empty_returns_empty() {
        todo!()
    }
}
