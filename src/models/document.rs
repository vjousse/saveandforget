use diesel::PgConnection;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use crate::schema::documents;

#[derive(Debug, Deserialize, PostgresMapper, Serialize)]
#[pg_mapper(table = "document")] // singular 'user' is a keyword..
#[derive(Queryable)]
pub struct Document {
    pub id: i64,
    pub filename: String,
    pub description: Option<String>,
}

impl Document {
    pub fn new(filename: String, description: String) -> Document {
        Document {
            id: 1,
            filename: filename,
            description: Some(description),
        }
    }
}

#[derive(Insertable, Deserialize, Debug)]
#[table_name = "documents"]
pub struct NewDocument {
    pub filename: String,
    pub description: Option<String>,
}

impl NewDocument {
    pub fn create(&self, connection: &PgConnection) -> Result<Document, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(documents::table)
            .values(self)
            .get_result(connection)
    }

    pub fn new(filename: String, description: Option<String>) -> NewDocument {
        NewDocument {
            filename: filename,
            description: description,
        }
    }
}

// This will tell the compiler that the struct will be serialized and
// deserialized, we need to install serde to make it work.
#[derive(Serialize, Deserialize)]
pub struct DocumentList(pub Vec<Document>);

impl DocumentList {
    pub fn list(connection: &PgConnection) -> Self {
        use crate::schema::documents::dsl::*;
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        let result = documents
            .limit(10)
            .load::<Document>(connection)
            .expect("Error loading documents");

        // We return a value by leaving it without a comma
        DocumentList(result)
    }
}
