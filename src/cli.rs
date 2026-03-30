use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "sage",
    about = "Index docs and code, then chat with an agent over them"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Index a documentation or source corpus into pgvector
    Index {
        #[command(subcommand)]
        source: IndexSource,
    },
    /// Start the interactive chat session
    Chat,
}

#[derive(Subcommand)]
pub enum IndexSource {
    /// Index an HTML file, chunked by heading boundaries
    Html {
        /// Logical name stored in the database. Defaults to the filename stem.
        #[arg(long)]
        source: Option<String>,
        #[arg(long)]
        path: PathBuf,
    },
    /// Index a PDF, chunked by section heading
    Pdf {
        /// Logical name stored in the database. Defaults to the filename stem.
        #[arg(long)]
        source: Option<String>,
        #[arg(long)]
        path: PathBuf,
    },
    /// Index plain-text or Markdown files, chunked by heading or paragraph
    Text {
        /// Logical name stored in the database. Defaults to the filename stem.
        #[arg(long)]
        source: Option<String>,
        #[arg(long)]
        path: PathBuf,
    },
    /// Index a source tree (C/C++ or Rust), chunked by function boundary
    Code {
        /// Logical name stored in the database (e.g. "my-project", "some-lib")
        #[arg(long)]
        source: String,
        #[arg(long)]
        path: PathBuf,
    },
}

pub fn parse() -> Cli {
    Cli::parse()
}
