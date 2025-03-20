use axum::{
    body::Body,
    extract::Path,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use std::path::Path as StdPath;

pub async fn html_dl() -> Result<axum::response::Html<String>, (axum::http::StatusCode, String)> {
    let mut html = String::from(
        r#"<!doctype html>
        <html>
        <head><title>Download Files</title></head>
        <body>
        <h1>Download Files</h1>
        <ul>"#,
    );
    let upload_path = StdPath::new("upload");
    for entry in std::fs::read_dir(upload_path)
        .map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        let entry =
            entry.map_err(|e| (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
        if entry.path().is_file() {
            let filename = entry.file_name().into_string().map_err(|_| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Invalid filename".to_string(),
                )
            })?;
            // Link to another route that will serve the file for download.
            html.push_str(&format!(
                "<li><a href=\"/download/file/{}\">{}</a></li>",
                filename, filename
            ));
        }
    }
    html.push_str(
        r#"</ul>
        </body>
        </html>"#,
    );
    Ok(axum::response::Html(html))
}

// has to be async
pub async fn download_file(Path(filename): Path<String>) -> impl IntoResponse {
    let file_path = StdPath::new("upload").join(&filename);
    if !file_path.exists() {
        return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
    }
    match std::fs::read(&file_path) {
        Ok(contents) => {
            let response = Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{}\"", filename),
                )
                .body(Body::from(contents))
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
            Ok(response)
        }
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
