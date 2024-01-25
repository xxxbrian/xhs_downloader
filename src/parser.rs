use std::collections::HashSet;

use regex::Regex;
use reqwest;
use scraper::{Html, Selector};

pub async fn fetch_original_image_url(
    url: &str,
) -> Result<(NoteType, Vec<String>), Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("script").unwrap();
    let mut note_type = NoteType::Normal;
    let mut image_urls = HashSet::new();

    for script in document.select(&selector) {
        let script_content = script.inner_html();
        if script_content.starts_with("window.__INITIAL_STATE__=") {
            let json_content = script_content
                .trim_start_matches("window.__INITIAL_STATE__=")
                .replace("undefined", "0")
                .replace("null", "0");
            let json: serde_json::Value = serde_json::from_str(&json_content)?;
            let first_note_id = json["note"]["firstNoteId"].as_str().unwrap();
            let note_info =
                json["note"]["noteDetailMap"].get(first_note_id).unwrap()["note"].clone();
            match note_info["type"].as_str().unwrap() {
                "normal" => {
                    note_type = NoteType::Normal;
                    let image_list = note_info["imageList"].as_array().unwrap();
                    for image in image_list {
                        let image_url = image["urlDefault"].as_str().unwrap();
                        image_urls.insert(image_url.to_string());
                    }
                }
                "video" => {
                    note_type = NoteType::Video;
                    let streams = note_info["video"]["media"]["stream"].clone();
                    for stream in streams.as_object().unwrap().values() {
                        match stream[0]["masterUrl"].as_str() {
                            Some(master_url) => {
                                image_urls.insert(master_url.to_string());
                            }
                            None => {}
                        }
                    }
                }
                _ => {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Unknown note type",
                    )));
                }
            }
            break;
        }
    }
    Ok((note_type, image_urls.into_iter().collect()))
}

pub fn get_image_token(image_url: String) -> String {
    // http://sns-webpic-qc.xhscdn.com/202401241638/e397dfb12da8a8d418432ddcfa3d551c/{token}!nd_dft_wlteh_jpg_3
    let re = Regex::new(r"/([^\/]+?)!").unwrap();
    let token = re
        .captures(image_url.as_str())
        .unwrap()
        .get(1)
        .unwrap()
        .as_str();
    token.to_string()
}

pub enum ImageType {
    Png,
    Jpg,
    Webp,
    Original,
}

pub enum NoteType {
    Normal,
    Video,
}

pub fn generate_image_links(
    original_image_urls: Vec<String>,
    image_type: ImageType,
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut image_links = Vec::new();

    for url in original_image_urls {
        let image_link = match image_type {
            ImageType::Png => format!(
                "https://ci.xiaohongshu.com/{}?imageView2/2/w/format/png",
                get_image_token(url)
            ),
            ImageType::Jpg => format!(
                "https://ci.xiaohongshu.com/{}?imageView2/2/w/format/jpg",
                get_image_token(url)
            ),
            ImageType::Webp => format!("https://sns-img-bd.xhscdn.com/{}", get_image_token(url)),
            ImageType::Original => url,
        };
        image_links.push(image_link);
    }

    Ok(image_links)
}
