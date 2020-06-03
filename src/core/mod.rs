use futures::future::join_all;
use reqwest::Client;
use std::path::Path;
use tokio::prelude::*;
use tokio::fs::File;
use uuid::Uuid;

pub type Url = String;

pub async fn download_files_join(urls: &Vec<Url>, destination_dir: &Path) -> Vec<Result<String, Box<dyn std::error::Error>>> {

    let client = Client::new();

    let downloading_futures =
        urls.iter()
            .map(|url| download_file(&client, url, destination_dir));

    join_all(downloading_futures).await
}

pub async fn download_file(client: &Client, url: &Url, destination_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {

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

            match File::create(&destination_file).await {
                Ok(mut file) => {
                    while let Some(chunk) = resp.chunk().await? {
                        file.write_all(&chunk).await?;
                    };
                    Ok(destination_file.to_str().unwrap_or("").to_owned())
                },
                Err(err) => Err(Box::new(err))
            }

        }
        Err(err) => {
            println!("Error downloading {:#?}", err);
            Err(Box::new(err))
        }
    }

}

pub fn extract_header(header_value: Option<&reqwest::header::HeaderValue>) -> Option<&str> {
    header_value.and_then(|v| v.to_str().ok())
}
