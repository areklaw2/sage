use crate::embedder::Embedder;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case", tag = "name", content = "input")]
pub enum Tool {
    SearchDocs {
        query: String,
        source: Option<String>,
    },
    SearchCode {
        query: String,
        source: Option<String>,
    },
    ReadFile {
        path: String,
    },
    FetchUrl {
        url: String,
    },
}

pub async fn execute_tool(
    pool: &PgPool,
    embedder: &Embedder,
    tool: Tool,
) -> anyhow::Result<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_deserialize_search_docs() {
        let json = r#"{"name":"search_docs","input":{"query":"memory layout","source":null}}"#;
        let tool: Tool = serde_json::from_str(json).unwrap();
        assert!(matches!(tool, Tool::SearchDocs { .. }));
    }

    #[test]
    fn test_tool_deserialize_search_code() {
        let json =
            r#"{"name":"search_code","input":{"query":"timer overflow","source":"my-project"}}"#;
        let tool: Tool = serde_json::from_str(json).unwrap();
        assert!(matches!(tool, Tool::SearchCode { .. }));
    }

    #[test]
    fn test_tool_deserialize_read_file() {
        let json = r#"{"name":"read_file","input":{"path":"/repos/my-project/src/timer.c"}}"#;
        let tool: Tool = serde_json::from_str(json).unwrap();
        assert!(matches!(tool, Tool::ReadFile { .. }));
    }
}
