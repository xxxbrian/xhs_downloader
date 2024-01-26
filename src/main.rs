use xhs_downloader::parser::{fetch_original_image_url, generate_image_links, ImageType, NoteType};
use xhs_downloader::webapi;
mod cli;
use cli::parse_args;
mod downloader;
use downloader::download_and_save;
use futures::future::join_all;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = parse_args();
    // webapi subcommand
    if let Some(webapi_arg) = matches.subcommand_matches("webapi") {
        let bind = webapi_arg
            .get_one::<String>("BIND")
            .map(|s| s.as_str())
            .unwrap_or("0.0.0.0:8000");
        webapi::run(bind).await;
        return Ok(());
    }
    // local download
    let url = matches
        .get_one::<String>("URL")
        .expect("URL is required")
        .as_str();
    let media_type = matches
        .get_one::<String>("type")
        .map(|s| s.as_str())
        .unwrap_or("ori");
    let output = matches
        .get_one::<String>("output")
        .map(|s| s.as_str())
        .unwrap_or(".");

    // fetch download links
    let (note_type, original_image_urls) = fetch_original_image_url(url).await.unwrap();
    let mut image_type = match media_type {
        "png" => ImageType::Png,
        "jpg" => ImageType::Jpg,
        "webp" => ImageType::Webp,
        _ => ImageType::Original,
    };
    // override image_type if note_type is video
    match note_type {
        NoteType::Normal => {}
        NoteType::Video => {
            image_type = ImageType::Original;
        }
    }

    // download at the same time
    let links = generate_image_links(original_image_urls, image_type).unwrap();
    let futures = links
        .into_iter()
        .map(|link| async move{
            let url = link.as_str();
            let file_path = output;
            return download_and_save(url, file_path).await
        });
    let results = join_all(futures).await;
    for result in results {
        result?;
    }
    Ok(())
}
