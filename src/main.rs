use tokio;
use xhs_downloader::{fetch_image_tokens, generate_image_links, ImageType};

#[tokio::main]
async fn main() {
    let url = "https://www.xiaohongshu.com/explore/65ae7093000000001100687c";
    let image_tokens = fetch_image_tokens(url).await.unwrap();
    let image_links = generate_image_links(image_tokens, ImageType::Webp).unwrap();
    println!("{:?}", image_links);
}