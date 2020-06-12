use actix_web::error as AWError;
use derive_more::{Display, From};
use std::error;

#[derive(Debug, Display, From)]
pub struct FileDownloadError {
    pub message: String,
}

impl error::Error for FileDownloadError {}

#[derive(Debug, Display, From, Serialize)]
pub struct DatabaseError {
    pub message: String,
}

impl error::Error for DatabaseError {}
impl AWError::ResponseError for DatabaseError {}
