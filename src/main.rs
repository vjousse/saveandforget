extern crate saveandforget as saf;
extern crate dotenv;
use std::path::Path;
use std::env;


use dotenv::dotenv;
// use saf::models::Document;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");
    let test_event = saf::messenger::get_full_test_event();

    let download_path = env::var("DOWNLOAD_PATH").expect("DOWNLOAD_PATH not found");

    let path = Path::new(&download_path);

    let urls = match saf::messenger::parse_document(test_event) {
        Ok(urls) => {
            let files:Vec<Result<String,Box<dyn std::error::Error>>> =
                saf::core::download_files_join(&urls, path).await;
            files
        },
        Err(_) => vec![]
    };




    dbg!(urls);
}
