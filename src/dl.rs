use axum::{
    body::Body, // Modified: StreamBody for streaming
    extract::Path,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};
use std::path::Path as StdPath;
use tokio::fs::{self, File as TokioFile}; // Added: tokio::fs and TokioFile
use tokio_util::io::ReaderStream; // Added: ReaderStream

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

    // Use tokio::fs::read_dir for async directory reading
    let mut entries = fs::read_dir(upload_path).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read upload directory: {e}"),
        )
    })?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read directory entry: {e}"),
        )
    })? {
        let path = entry.path();
        // Use tokio::fs::metadata to check if it's a file asynchronously
        if let Ok(metadata) = fs::metadata(&path).await {
            if metadata.is_file() {
                if let Some(filename_osstr) = path.file_name() {
                    // Attempt to convert OsStr to &str, handling potential errors
                    if let Some(filename_str) = filename_osstr.to_str() {
                        // URL encode the filename for the href attribute
                        let encoded_filename = urlencoding::encode(filename_str);
                        html.push_str(&format!(
                            "<li><a href=\"/download/file/{encoded_filename}\">{filename_str}</a></li>" // Display original, link encoded
                        ));
                    } else {
                        // Log or handle filenames that are not valid UTF-8
                        eprintln!("Skipping non-UTF8 filename: {filename_osstr:?}");
                    }
                }
            }
        } else {
            eprintln!("Failed to get metadata for path: {path:?}");
        }
    }

    html.push_str(
        r#"</ul>
        </body>
        </html>"#,
    );
    Ok(axum::response::Html(html))
}

pub async fn download_file(Path(filename): Path<String>) -> impl IntoResponse {
    // Path extractor automatically URL decodes the segment.
    let file_path = StdPath::new("upload").join(&filename);

    // Asynchronously check if the file exists and is a file
    match fs::metadata(&file_path).await {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err((
                    StatusCode::NOT_FOUND,
                    "Requested path is not a file".to_string(),
                ));
            }
        }
        Err(_) => {
            return Err((StatusCode::NOT_FOUND, "File not found".to_string()));
        }
    }

    match TokioFile::open(&file_path).await {
        Ok(file) => {
            let stream = ReaderStream::new(file);
            // axum's StreamBody doesn't exist anymore
            // we use Body::from_stream instead
            let body = Body::from_stream(stream);

            let response = Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                .header(
                    header::CONTENT_DISPOSITION,
                    format!("attachment; filename=\"{filename}\""), // Original filename here
                )
                .body(body)
                .map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Error building response: {e}"),
                    )
                })?;
            Ok(response)
        }
        Err(e) => {
            eprintln!("Error opening file {file_path:?} for download: {e}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error serving file".to_string(),
            ))
        }
    }
}
