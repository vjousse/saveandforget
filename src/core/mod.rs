use reqwest::Client;
use std::path::Path;
use tokio::prelude::*;
use tokio::fs::File;
use uuid::Uuid;

pub type Url = String;

pub async fn add_new_file(_file: &Path) -> () {

}

pub async fn download_files(urls: &Vec<Url>, destination_dir: &Path) -> Result<i32, Box<dyn std::error::Error>> {

    // https://users.rust-lang.org/t/solved-problem-with-reqwest-0-10-async-client/36373/13
    // http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust
    let client = Client::new();
    for url in urls.iter() {

        dbg!(format!("Downloading {}", &url));

        match client.get(url).send().await {

            Ok(mut resp) => {
                dbg!(format!("Downloaded {}", &url));
                let content_type:Option<&str> = extract_header(resp.headers().get("content-type"));
                dbg!(content_type);

                let my_uuid = Uuid::new_v4();
                let destination_file = destination_dir.join(my_uuid.to_string() +
                match content_type {
                    Some("image/png") => ".png",
                    Some("image/jpeg") => ".jpg",
                    Some("image/gif") => ".gif",
                    Some(_) => "",
                    None => ""
                });

                dbg!(&destination_file);

                match File::create(destination_file).await {
                    Ok(mut file) => {
                        while let Some(chunk) = resp.chunk().await? {
                            file.write_all(&chunk).await?;
                        }
                    },
                    Err(_) => ()
                }

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
