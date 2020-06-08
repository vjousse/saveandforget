use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "document")] // singular 'user' is a keyword..
pub struct Document {
    pub filename: String,
    pub description: String,
}

impl Document {
    pub fn new(filename: String, description: String) -> Document {
        Document {
            filename: filename,
            description: description,
        }
    }
}
