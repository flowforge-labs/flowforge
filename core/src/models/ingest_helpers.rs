use std::path::PathBuf;
use thiserror;
#[derive(thiserror::Error, Debug)]

// TODO: Figure out the damn project structure.
pub enum IngestError {
    #[error("Failed to read file.")]
    UnableToReadFile(PathBuf),
    #[error("File path does not exist.")]
    PathDoesNotExist(PathBuf),
    #[error("Parse failed for unspecified reason")]
    ParseFailed(String),
}

pub struct ParsedFile {
    pub path: PathBuf,
    pub lines_read: usize,
    pub events_parsed: usize,
    pub errors: Vec<IngestError>,
}

pub struct IngestResult {
    pub total_files: usize,
    pub total_lines: usize,
    pub total_events: usize,
    pub files: Vec<ParsedFile>,
}
