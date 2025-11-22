pub fn ingest_file(filepath: &Path) -> Result<Vec<LogEvent>, Ingest>{todo!()}

pub fn ingest_folder(filepath: &Path) -> Result<Vec<LogEvent>, IngestError>{todo!()}

pub fn detect_log_type(logline: &str) -> LogType {todo!()}

pub fn parse_line(line: &str, line_number: usize, source_file: &Path) -> Result<LogEvent, IngestError> {todo!()}

#[derive(Debug, thiserror::Error)]
enum IngestError {
    UnableToReadFile(PathBuf),
    PathDoesNotExist(PathBuf),
    ParseFailed(String),
}

