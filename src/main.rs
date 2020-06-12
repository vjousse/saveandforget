extern crate dotenv;
#[macro_use]
extern crate log;
extern crate saveandforget as saf;


use actix_web::{
    error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer
};

use db::PgPool;
use dotenv::dotenv;
use saf::db;
use saf::models::document::{DocumentList};
use saf::web_handlers::facebook;
use std::path::PathBuf;

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

/*
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
*/


/*
async fn index() -> &'static str {
    "Save and forget"
}
*/


async fn index(_req: HttpRequest, pg_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    let documents= 
        DocumentList::list(&pg_pool, None)
            .map_err(|e| {
                error!("Database error {}", e.message);
                error::ErrorInternalServerError("Database error.")
            })?;

    Ok(HttpResponse::Ok().json(documents))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init();

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "localhost:8000".to_string());

    debug!("Webserver listening on {}", server_address);

    let app = move || {

        let config = crate::config::Config::from_env().expect("Error reading .env file.");

        let pg_pool:PgPool =
            db::init_pool(&config.database_url)
                .expect("Failed to create pool");
        App::new()
            .data(pg_pool)
            .data(saf::models::web::AppState {
                fb_verify_token: config.fb_verify_token.clone(),
                download_path: PathBuf::from(config.download_path)
            })
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/fb")
                    .route("/webhook", web::get().to(facebook::fb_webhook_hub))
                    .route("/webhook", web::post().to(facebook::fb_webhook_event))
            )
            .service(web::resource("/").to(index))
    };


    debug!("Starting server");
    HttpServer::new(app).bind(server_address)?.run().await
}
