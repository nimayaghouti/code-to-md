use std::path::{Path, PathBuf};

const MAX_PREVIEW_SIZE: u64 = 1024 * 1024; // 1 MiB

#[derive(Debug, Clone)]
pub struct FilePreview {
    pub path: PathBuf,
    pub size: u64,
    pub content: PreviewContent,
}

#[derive(Debug, Clone)]
pub enum PreviewContent {
    Text(String),
    TooLarge,
    Binary,
    Error(String),
}

pub fn load_preview(path: &Path) -> anyhow::Result<FilePreview> {
    let metadata = std::fs::metadata(path)?;
    let size = metadata.len();

    if size > MAX_PREVIEW_SIZE {
        return Ok(FilePreview {
            path: path.to_path_buf(),
            size,
            content: PreviewContent::TooLarge,
        });
    }

    let bytes = std::fs::read(path)?;

    // Check for null bytes in first 8KB
    let check_len = bytes.len().min(8192);
    if bytes[..check_len].contains(&0) {
        return Ok(FilePreview {
            path: path.to_path_buf(),
            size,
            content: PreviewContent::Binary,
        });
    }

    // Try to decode as UTF-8
    match String::from_utf8(bytes) {
        Ok(text) => Ok(FilePreview {
            path: path.to_path_buf(),
            size,
            content: PreviewContent::Text(text),
        }),
        Err(_) => Ok(FilePreview {
            path: path.to_path_buf(),
            size,
            content: PreviewContent::Binary,
        }),
    }
}
