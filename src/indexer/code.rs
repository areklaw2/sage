use std::path::Path;

pub struct CodeChunk {
    pub source: String,
    pub file: String,
    pub fn_name: String,
    pub line_start: i32,
    pub line_end: i32,
    pub content: String,
}

/// Source language supported by the code indexer.
pub enum Language {
    C,
    Rust,
}

impl Language {
    /// Infer language from file extension. Returns `None` for unrecognised extensions.
    pub fn detect(path: &Path) -> Option<Self> {
        match path.extension()?.to_str()? {
            "c" | "h" | "cpp" | "cc" | "cxx" => Some(Self::C),
            "rs" => Some(Self::Rust),
            _ => None,
        }
    }
}

/// Walk a source tree and chunk by function boundary using tree-sitter.
/// Supports C/C++ (*.c, *.h, *.cpp) and Rust (*.rs); other extensions are skipped.
/// Preceding block/line comments are attached to each function chunk.
pub fn index_code_repo(source_name: &str, root: &Path) -> anyhow::Result<Vec<CodeChunk>> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detect_c() {
        assert!(matches!(
            Language::detect(Path::new("timer.c")),
            Some(Language::C)
        ));
        assert!(matches!(
            Language::detect(Path::new("util.h")),
            Some(Language::C)
        ));
    }

    #[test]
    fn test_language_detect_rust() {
        assert!(matches!(
            Language::detect(Path::new("main.rs")),
            Some(Language::Rust)
        ));
    }

    #[test]
    fn test_language_detect_unknown_returns_none() {
        assert!(Language::detect(Path::new("readme.md")).is_none());
    }

    #[test]
    fn test_index_code_repo_nonexistent_path_errors() {
        todo!()
    }

    #[test]
    fn test_index_code_repo_empty_dir_returns_empty() {
        todo!()
    }
}