extern crate saveandforget as saf;
use std::path::Path;

// use saf::models::Document;

#[tokio::main]
async fn main() {
    let test_event = saf::messenger::get_full_test_event();
    let path = Path::new("/home/vjousse/usr/src/saveandforget/saveandforget/downloads/");
    let result = match saf::messenger::parse_document(test_event) {
        Ok(urls) => {

            let files:Vec<Result<String,Box<dyn std::error::Error>>> =
                saf::core::download_files_join(&urls, path).await;

            dbg!(&files);
            Ok(files.len())
        },
        Err(_) => Err(0),
    };




    println!("{:#?}", result);
}
