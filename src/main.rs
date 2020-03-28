extern crate saveandforget as saf;

fn main() {
    let test_event = saf::messenger::get_full_test_event();
    let _ = saveandforget::messenger::parse_document(test_event);
    println!("Hello, world!");
}
