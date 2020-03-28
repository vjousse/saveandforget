extern crate saveandforget as saf;

fn main() {
    let test_event = saf::messenger::get_full_test_event();
    match saf::messenger::parse_document(test_event) {
        Ok(urls) => saf::core::download_files(&urls),
        Err(_) => (),
    };

    println!("Hello, world!");
}
