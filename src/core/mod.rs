use std::collections::HashMap;

pub type Url = String;

pub async fn download_files(urls: &Vec<Url>) -> Result<i32, Box<dyn std::error::Error>> {

    // https://users.rust-lang.org/t/solved-problem-with-reqwest-0-10-async-client/36373/13
    // http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust
    for url in urls.iter() {
        let resp = reqwest::get("https://httpbin.org/ip")
            .await?
            .json::<HashMap<String, String>>()
            .await?;
        println!("{:#?}", resp);
    }
    Ok(1)
}
