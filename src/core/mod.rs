use crate::errors::FileDownloadError;
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
            let file_extension = get_file_extension(resp.headers(), Some(url))?;

            dbg!(&file_extension);

            let my_uuid = Uuid::new_v4();
            let destination_file = destination_dir.join(format!("{}{}", my_uuid.to_string(), file_extension));

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

pub fn get_file_extension(headers: &reqwest::header::HeaderMap, url: Option<&Url>) -> Result<String, Box<dyn std::error::Error>> {

    let header: Option<&str> = extract_header(headers.get("content-type"));

    match header {
        Some(content_type) => match content_type {
            "image/png" => Ok(".png".to_owned()),
            "image/jpeg" => Ok(".jpg".to_owned()),
            "image/gif" => Ok(".gif".to_owned()),
            _ => {
                Err(Box::new(FileDownloadError {
                    message : format!(
                      "Not an image extension, got unknown extension {}{}",
                      content_type, match url {
                        Some(u) => format!(". Url was: {}", u),
                        None => "".to_owned()
                    })
                }))
            }
        }
        None => Err(
                Box::new(FileDownloadError {
                    message : "Content type not found in headers".to_owned()
            }))
    }
}
