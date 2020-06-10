#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate saveandforget as saf;

use db_connection::PgPooledConnection;
use dotenv::dotenv;
use saf::models::document::NewDocument;
use std::path::Path;
use std::env;

pub mod db_connection;
pub mod schema;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    let test_event = saf::messenger::get_full_test_event();

    let download_path = env::var("DOWNLOAD_PATH").expect("DOWNLOAD_PATH not found");
    let pg_connection_pool:PgPooledConnection =
        db_connection::get_connection_pool()
            .get()
            .expect("Impossible to connect to DATABASE");

    let path = Path::new(&download_path);

    let urls = match saf::messenger::parse_document(test_event) {
        Ok(urls) => {
            let files:Vec<Result<String,Box<dyn std::error::Error>>> =
                saf::core::download_files(&urls, path).await;
            files
        },
        Err(_) => vec![]
    };

    let documents = 
        urls
            .iter()
            .map(
            // Map over the Vec of destination files
                |result| result.as_ref().map(
                    // Convert Result<String,_> into Result<Document,_>
                    |download_path| NewDocument::new(download_path.to_owned(), Some("Test description".to_owned()))
                ).map(|new_doc| new_doc.create(&pg_connection_pool))
            )
            .collect::<Vec<_>>();




    dbg!(documents);
}
