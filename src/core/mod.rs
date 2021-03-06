use futures::future::join_all;
use reqwest::Client;
use std::path::Path;
use std::io;
use std::fs;
use tokio::prelude::*;
use tokio::fs::File;
use uuid::Uuid;

use crate::errors::SafError;

pub type Url = String;

pub fn rm_file(filename: &str, destination_dir: &Path) -> io::Result<()> {
    fs::remove_file(destination_dir.join(&filename))
}

pub async fn download_files(urls: &Vec<Url>, destination_dir: &Path) -> Vec<Result<String, Box<dyn std::error::Error>>> {

    let client = Client::new();

    // Transform a collection of Urls into a collection of futures that
    // will save the files on disk
    let downloading_futures =
        urls.iter()
            .map(|url| download_file(&client, url, destination_dir));

    // Excute all the futures async
    join_all(downloading_futures).await
}

pub async fn download_file(client: &Client, url: &Url, destination_dir: &Path) -> Result<String, Box<dyn std::error::Error>> {

    match client.get(url).send().await {

        Ok(mut resp) => {
            debug!("Trying to download {}", &url);

            let file_extension = get_file_extension(resp.headers(), Some(url))?;

            let my_uuid = Uuid::new_v4();
            let destination_filename = format!("{}{}", my_uuid.to_string(), file_extension);
            let destination_file = destination_dir.join(&destination_filename);

            debug!("Saving file to {:?}", &destination_file);

            match File::create(&destination_file).await {
                Ok(mut file) => {
                    while let Some(chunk) = resp.chunk().await? {
                        file.write_all(&chunk).await?;
                    };
                    Ok(destination_filename)
                },
                Err(err) => Err(Box::new(err))
            }

        }
        Err(err) => {
            error!("Error downloading {:#?}", err);
            Err(Box::new(err))
        }
    }

}

pub fn extract_header(header_value: Option<&reqwest::header::HeaderValue>) -> Option<&str> {
    header_value.and_then(|v| v.to_str().ok())
}

pub fn get_file_extension(headers: &reqwest::header::HeaderMap, url: Option<&Url>) -> Result<String, SafError> {

    let header: Option<&str> = extract_header(headers.get("content-type"));

    match header {
        Some(content_type) => match content_type {
            "image/png" => Ok(".png".to_owned()),
            "image/jpeg" => Ok(".jpg".to_owned()),
            "image/gif" => Ok(".gif".to_owned()),
            _ => {
                Err(SafError::FileDownloadError {
                    message : format!(
                      "Not an image extension, got unknown extension {}{}",
                      content_type, match url {
                        Some(u) => format!(". Url was: {}", u),
                        None => "".to_owned()
                    })
                })
            }
        }
        None => Err(
                SafError::FileDownloadError {
                    message : "Content type not found in headers".to_owned()
            })
    }
}
