use xhs_downloader::webapi;

#[tokio::main]
async fn main() {
    webapi::run("0.0.0.0:8880").await;
}
