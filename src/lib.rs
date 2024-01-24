use std::collections::HashSet;

use reqwest;
use scraper::{Html, Selector};
use regex::Regex;

pub async fn fetch_image_tokens(url: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("meta").unwrap();
    let mut image_tokens = HashSet::new();

    // http://sns-webpic-qc.xhscdn.com/202401241638/e397dfb12da8a8d418432ddcfa3d551c/{token}!nd_dft_wlteh_jpg_3
    let re = Regex::new(r"/([^\/]+?)!").unwrap();

    // iterate over all meta elements
    for element in document.select(&selector) {
        if element.value().attr("name") == Some("og:image") {
            if let Some(content) = element.value().attr("content") {
                if let Some(captures) = re.captures(content) {
                    if let Some(token) = captures.get(1) {
                        image_tokens.insert(token.as_str().to_string());
                    }
                }
            }
        }
    }

    Ok(image_tokens.into_iter().collect())
}

pub enum ImageType {
    Png,
    Webp,
}

pub fn generate_image_links(tokens: Vec<String>, image_type: ImageType) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut image_links = Vec::new();

    for token in tokens {
        let image_link = match image_type {
            ImageType::Png => format!("https://ci.xiaohongshu.com/{}?imageView2/2/w/format/png", token),
            ImageType::Webp => format!("https://sns-img-bd.xhscdn.com/{}", token),
        };
        image_links.push(image_link);
    }

    Ok(image_links)
}
