use reqwest;
use std::error::Error;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn download_and_save(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_name = url.split('/').last().unwrap();
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request Faild, status: {}", response.status()),
        )));
    }

    let bytes = response.bytes().await?;

    let file_path = Path::new(file_path).join(file_name);
    let mut file = File::create(file_path).await?;
    file.write_all(&bytes).await?;

    println!("Downloaded: {}", file_name);
    Ok(())
}
