use actix_web::error as AWError;
use bcrypt::BcryptError;
use derive_more::{Display, From};
use diesel::result;
use std::error;
use std::fmt;

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

#[derive(Debug, Display, From, PartialEq)]
pub struct MessengerError {
    pub message: String,
}

impl error::Error for MessengerError {}
impl AWError::ResponseError for MessengerError {}

#[derive(Debug, Display, PartialEq)]
pub enum SafError {
    FileDownloadError { message: String },
    BadObjectError { object: String },
    EventParsingError { message: String },
}

impl error::Error for SafError {}
impl AWError::ResponseError for SafError {}

pub enum MyStoreError {
    HashError(BcryptError),
    DBError(result::Error),
    PasswordNotMatch(String),
    WrongPassword(String),
}

// We need this to performs a conversion from BcryptError to MyStoreError
impl From<BcryptError> for MyStoreError {
    fn from(error: BcryptError) -> Self {
        MyStoreError::HashError(error)
    }
}

// We need this to performs a conversion from diesel::result::Error to MyStoreError
impl From<result::Error> for MyStoreError {
    fn from(error: result::Error) -> Self {
        MyStoreError::DBError(error)
    }
}

// We need this so we can use the method to_string over MyStoreError
impl fmt::Display for MyStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyStoreError::HashError(error) => write!(f, "{}", error),
            MyStoreError::DBError(error) => write!(f, "{}", error),
            MyStoreError::PasswordNotMatch(error) => write!(f, "{}", error),
            MyStoreError::WrongPassword(error) => write!(f, "{}", error),
        }
    }
}
