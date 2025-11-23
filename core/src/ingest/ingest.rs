pub fn ingest_folder(path: PathBuf) -> Result<IngestResult, IngestError> {
    if!path.exists() {
        return Err(IngestError::PathDoesNotExist(path));
    } 

    Ok(vec![])
}