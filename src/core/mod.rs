use std::collections::HashMap;

pub type Url = String;

pub async fn download_files(_urls: &Vec<Url>) -> Result<i32, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(1)
}
