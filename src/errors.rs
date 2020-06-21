use actix_web::error as AWError;
use bcrypt::BcryptError;
use derive_more::{Display, From};
use diesel::result;
use std::error;

#[derive(Debug, Display, From, PartialEq)]
pub struct MessengerError {
    pub message: String,
}

impl error::Error for MessengerError {}
impl AWError::ResponseError for MessengerError {}

#[derive(Debug, Display)]
pub enum SafError {
    HashError(BcryptError),
    ConnectionError(String),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String),
    FileDownloadError { message: String },
    BadObjectError { object: String },
    EventParsingError { message: String },
}

// We need this to performs a conversion from BcryptError to MyStoreError
impl From<BcryptError> for SafError {
    fn from(error: BcryptError) -> Self {
        SafError::HashError(error)
    }
}

// We need this to performs a conversion from diesel::result::Error to MyStoreError
impl From<result::Error> for SafError {
    fn from(error: result::Error) -> Self {
        SafError::DBError(error)
    }
}

impl error::Error for SafError {}
impl AWError::ResponseError for SafError {}
