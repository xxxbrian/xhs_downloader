use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

use super::parser::{fetch_original_image_url, generate_image_links, ImageType, NoteType};

pub async fn run(bind: &str) {
    // build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "xhs_downloader" }))
        .route("/fetch_media_links", get(fetch_media_links))
        .route("/echo", post(echo));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// custom error type that will be returned by `fetch_media_links`
#[derive(Debug)]
struct MediaLinksError {
    message: String,
}

impl fmt::Display for MediaLinksError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for MediaLinksError {}

impl IntoResponse for MediaLinksError {
    fn into_response(self) -> Response {
        let body = Json(serde_json::json!({ "error": self.message }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Deserialize)]
struct MediaLinksQuery {
    url: String,
    media_type: String,
}

async fn fetch_media_links(
    Query(MediaLinksQuery { url, media_type }): Query<MediaLinksQuery>,
) -> Result<Json<Vec<String>>, MediaLinksError> {
    println!("url: {}, media_type: {}", url, media_type);
    let (note_type, original_image_urls) =
        fetch_original_image_url(&url)
            .await
            .map_err(|e| MediaLinksError {
                message: format!("Error fetching image tokens: {}", e),
            })?;
    let mut image_type = match media_type.as_str() {
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
    let image_links =
        generate_image_links(original_image_urls, image_type).map_err(|e| MediaLinksError {
            message: format!("Error generating image links: {}", e),
        })?;
    Ok(Json(image_links))
}

#[derive(Serialize, Deserialize)]
struct SettingJson {
    username: String,
}

async fn echo(Json(payload): Json<SettingJson>) -> (StatusCode, Json<SettingJson>) {
    (StatusCode::CREATED, Json(payload))
}
