use uuid::Uuid;
use std::path::Path;
use reqwest::Client;

pub type Url = String;

pub async fn download_files(urls: &Vec<Url>, destination_dir: &Path) -> Result<i32, Box<dyn std::error::Error>> {

    // https://users.rust-lang.org/t/solved-problem-with-reqwest-0-10-async-client/36373/13
    // http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust
    let client = Client::new();
    for url in urls.iter() {
        match client.get(url).send().await {
            Ok(resp) => {
                println!("{:#?}", resp);

                let content_type:Option<&str> = extract_header(resp.headers().get("content-type"));
                
                let my_uuid = Uuid::new_v4();
                let destination_file = destination_dir.join(my_uuid.to_string() +
                match content_type {
                    Some("image/png") => ".png",
                    Some("image/jpeg") => ".jpg",
                    Some("image/gif") => ".gif",
                    Some(_) => "",
                    None => ""
                });

                println!("Destination file: {:#?}", destination_file);
            }
            Err(err) => {
                println!("Error downloading {:#?}", err);
            }
        }
    }
    Ok(1)
}

pub fn extract_header(header_value: Option<&reqwest::header::HeaderValue>) -> Option<&str> {
    header_value.and_then(|v| v.to_str().ok())
}
