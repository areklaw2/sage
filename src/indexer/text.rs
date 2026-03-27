use std::path::Path;

pub struct TextChunk {
    pub source: String,
    pub file: String,
    pub section: Option<String>,
    pub content: String,
}

/// Walk a directory (or read a single file) and chunk plain-text and Markdown files
/// (*.txt, *.md). Markdown files are chunked by `#` heading boundaries; plain-text
/// files are chunked by double-newline paragraph breaks.
pub fn index_text(source_name: &str, path: &Path) -> anyhow::Result<Vec<TextChunk>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_text_chunk_has_section_field() {
        let chunk = TextChunk {
            source: "docs".to_string(),
            file: "guide.md".to_string(),
            section: Some("Installation".to_string()),
            content: "Run cargo install sage".to_string(),
        };
        assert_eq!(chunk.source, "docs");
        assert_eq!(chunk.section.as_deref(), Some("Installation"));
    }

    #[test]
    fn test_index_text_nonexistent_path_errors() {
        todo!()
    }

    #[test]
    fn test_index_text_empty_dir_returns_empty() {
        todo!()
    }
}
