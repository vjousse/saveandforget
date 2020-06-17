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

#[derive(Debug, Display, From)]
pub struct MessengerError {
    pub message: String,
}

impl error::Error for MessengerError {}
impl AWError::ResponseError for MessengerError {}

#[derive(Debug, Display)]
pub enum SafError {
    FileDownloadError { message: String },
    BadObjectError { object: String },
    EventParsingError { message: String },
}

impl error::Error for SafError {}
impl AWError::ResponseError for SafError {}
