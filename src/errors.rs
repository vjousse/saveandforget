use deadpool_postgres::PoolError;
use derive_more::{Display, From};
use std::error;
use std::fmt;
use tokio_pg_mapper::Error as PGMError;
use tokio_postgres::error::Error as PGError;

#[derive(Display, From, Debug)]
pub enum SafError {
    NotFound,
    PGError(PGError),
    PGMError(PGMError),
    PoolError(PoolError),
}
impl std::error::Error for SafError {}

#[derive(Debug)]
pub struct FileDownloadError {
    pub message: String,
}

impl error::Error for FileDownloadError {}

impl fmt::Display for FileDownloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
