use xhs_downloader::webapi;
mod cli;
use cli::parse_args;

#[tokio::main]
async fn main() {
    let matches = parse_args();
    // webapi subcommand
    if let Some(webapi_arg) = matches.subcommand_matches("webapi") {
        let bind = webapi_arg.get_one::<String>("BIND").map(|s| s.as_str()).unwrap_or("0.0.0.0:8000");
        webapi::run(bind).await;
        return;
    }
    // local download
    let url = matches.get_one::<String>("URL").expect("URL is required").as_str();
    let download_type = matches.get_one::<String>("type").map(|s| s.as_str()).unwrap_or("ori");
    let output = matches.get_one::<String>("output").map(|s| s.as_str()).unwrap_or(".");
    println!("url: {}, type: {}, output: {}", url, download_type, output);
    // TODO: parse and download
}
