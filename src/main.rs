#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate saveandforget as saf;

use db_connection::PgPooledConnection;
use dotenv::dotenv;
use saf::models::document::NewDocument;
use std::path::{Path, PathBuf};

pub mod db_connection;
pub mod schema;

struct AppState {
    fb_verify_token: String,
    download_path: PathBuf,
    pg_pool: PgPooledConnection
}


mod config {
    pub use ::config::ConfigError;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct Config {
        pub fb_verify_token: String,
        pub download_path: String,
        pub database_url: String,
    }
    impl Config {
        pub fn from_env() -> Result<Self, ConfigError> {
            let mut cfg = ::config::Config::new();
            cfg.merge(::config::Environment::new())?;
            cfg.try_into()
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    let test_event = saf::messenger::get_full_test_event();
    let config = crate::config::Config::from_env().unwrap();

    let pg_connection_pool:PgPooledConnection =
        db_connection::get_connection_pool(config.database_url)
            .get()
            .expect("Impossible to connect to DATABASE");

    let path = Path::new(&config.download_path);

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
