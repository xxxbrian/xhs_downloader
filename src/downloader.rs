use mime_guess;
use reqwest;
use std::error::Error;
use std::path::Path;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn download_and_save(url: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let file_name = url.split('/').last().unwrap();
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Request Faild, status: {}", response.status()),
        )));
    }

    let headers = response.headers().clone();
    let bytes = response.bytes().await?;

    let content_type = headers
        .get(reqwest::header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or_default();

    let mut file_name_with_extension = String::from(file_name);

    println!("content_type: {}", content_type);
    // check if the file name already has an extension
    if Path::new(file_name).extension().is_none() {
        let extension = content_type.split('/').last().unwrap_or("tmp");
        let mime_guess = mime_guess::from_ext(extension);
        let mime_type = mime_guess
            .first()
            .map(|mime| mime.essence_str().to_string())
            .unwrap_or("application/octet-stream".to_string());

        // append the extension to the file name
        file_name_with_extension = format!(
            "{}.{}",
            file_name,
            mime_type.split('/').last().unwrap_or("tmp")
        );
    }

    let file_path = Path::new(file_path).join(file_name_with_extension);
    let mut file = File::create(file_path).await?;
    file.write_all(&bytes).await?;

    println!("Downloaded: {}", file_name);
    Ok(())
}
