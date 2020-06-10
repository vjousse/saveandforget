use derive_more::{Display, From};
use std::error;

#[derive(Debug, Display, From)]
pub struct FileDownloadError {
    pub message: String,
}

impl error::Error for FileDownloadError {}
