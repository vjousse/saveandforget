extern crate saveandforget as saf;
use futures::executor::block_on;
use std::path::Path;

use saf::models::Document;

#[tokio::main]
async fn main() {
    let test_event = saf::messenger::get_full_test_event();
    let path = Path::new("/home/vjousse/usr/src/saveandforget/saveandforget/downloads/");
    let result = match saf::messenger::parse_document(test_event) {
        Ok(urls) => {
            match saf::core::download_files(&urls, path).await {
                //@TODO: Get the paths of the downloaded files in return to 
                //insert a document in the database
                Ok(ok) => Ok(urls.len()),
                Err(e) => Err(0),

            }
        },
        Err(_) => Ok(0),
    };




    println!("{:#?}", result);
}
