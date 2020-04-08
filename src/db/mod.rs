use crate::{errors::SafError, models::Document};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn add_document(client: &Client, document_info: Document) -> Result<Document, SafError> {
    let _stmt = include_str!("../../sql/add_document.sql");
    let _stmt = _stmt.replace("$table_fields", &Document::sql_table_fields());
    let stmt = client.prepare(&_stmt).await.unwrap();

    client
        .query(
            &stmt,
            &[
                &document_info.filename,
                &document_info.description,
            ],
        )
        .await?
        .iter()
        .map(|row| Document::from_row_ref(row).unwrap())
        .collect::<Vec<Document>>()
        .pop()
        .ok_or(SafError::NotFound) // more applicable for SELECTs
}
