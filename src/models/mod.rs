use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "document")] // singular 'user' is a keyword..
pub struct Document {
    pub filename: String,
    pub description: String,
}
