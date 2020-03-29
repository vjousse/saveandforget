extern crate saveandforget as saf;
use futures::executor::block_on;
use std::path::Path;

#[tokio::main]
async fn main() {
    let test_event = saf::messenger::get_full_test_event();
    let path = Path::new("/home/vjousse/usr/src/saveandforget/saveandforget/downloads/");
    let result = match saf::messenger::parse_document(test_event) {
        Ok(urls) => block_on(saf::core::download_files(&urls, path)),
        Err(_) => Ok(0),
    };

    println!("{:#?}", result);
}
