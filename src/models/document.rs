use diesel::{PgConnection, QueryDsl, RunQueryDsl};

use crate::db;
use crate::db::PgPool;
use crate::errors::SafError;
use crate::schema::documents;
use chrono::Local;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Queryable, Serialize)]
pub struct Document {
    pub id: i64,
    pub filename: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

impl Document {
    pub fn find(id: &i64, connection: &PgConnection) -> Result<Document, diesel::result::Error> {
        documents::table.find(id).first(connection)
    }

    pub fn destroy(id: &i64, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        diesel::delete(documents::table.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(
        id: &i64,
        new_document: &NewDocument,
        connection: &PgConnection,
    ) -> Result<(), diesel::result::Error> {
        diesel::update(documents::table.find(id))
            .set(new_document)
            .execute(connection)?;
        Ok(())
    }
}

#[derive(Insertable, Deserialize, Debug, AsChangeset)]
#[table_name = "documents"]
pub struct NewDocument {
    pub filename: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

impl NewDocument {
    pub fn create(&self, pool: &PgPool) -> Result<Document, SafError> {
        diesel::insert_into(documents::table)
            .values(self)
            .get_result(&db::get_conn(pool)?)
            .map_err(|e| SafError::DBError(e))
    }

    pub fn new(filename: String, description: Option<String>) -> NewDocument {
        NewDocument {
            filename: filename,
            description: description,
            created_at: Local::now().naive_local(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DocumentList(pub Vec<Document>);

impl DocumentList {
    pub fn list(pool: &PgPool, limit: Option<i64>) -> Result<Self, SafError> {
        use crate::schema::documents::dsl::*;

        let mut query = documents.into_boxed();

        if let Some(nb) = limit {
            query = query.limit(nb);
        }

        let result = query
            .load::<Document>(&db::get_conn(pool)?)
            .expect("Error loading documents");

        Ok(DocumentList(result))
    }
}
