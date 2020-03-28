extern crate saveandforget as saf;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let test_event = saf::messenger::get_full_test_event();
    match saf::messenger::parse_document(test_event) {
        Ok(urls) => block_on(saf::core::download_files(&urls)),
        Err(_) => Ok(0),
    };

    println!("Hello, world!");
}
