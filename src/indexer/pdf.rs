use std::path::Path;

pub struct PdfChunk {
    pub source: String,
    pub section: String,
    pub content: String,
}

/// Extract text from a PDF and chunk by section heading.
/// Headings are detected by numeric patterns (e.g. "A4.3", "B1.2") or font-size heuristics.
/// The section heading is prepended to each chunk's content.
pub fn index_pdf(source_name: &str, path: &Path) -> anyhow::Result<Vec<PdfChunk>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_pdf_chunk_has_section_field() {
        let chunk = PdfChunk {
            source: "pdf".to_string(),
            section: "A4.3 Section".to_string(),
            content: "The ADD instruction...".to_string(),
        };
        assert_eq!(chunk.source, "pdf");
        assert_eq!(chunk.section, "A4.3 Section");
    }

    #[test]
    fn test_index_pdf_nonexistent_path_errors() {
        todo!()
    }
}
