use axum::{
    extract::Multipart,
    http::StatusCode,
    response::{Html, Redirect},
};
use std::path::Path as StdPath;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncWriteExt;

// This helper function maps the MultipartError to your function's error type.
fn map_multipart_error(e: axum::extract::multipart::MultipartError) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Multipart processing error: {e}"),
    )
}

pub async fn html_upload() -> Html<&'static str> {
    Html(
        r#"
        <!doctype html>
        <html>
        <head>
            <title>Upload File</title>
        </head>
        <body>
            <h1>Upload a File</h1>
            <form action="/upload" method="post" enctype="multipart/form-data">
                <input type="file" name="file" required>
                <input type="submit" value="Upload">
            </form>
        </body>
        </html>
        "#,
    )
}

pub async fn upload_file(
    mut multipart: Multipart, // Multipart needs to be mutable to call next_field()
) -> Result<Redirect, (StatusCode, String)> {
    // Loop through each field in the multipart stream
    while let Some(mut field) = multipart.next_field().await.map_err(map_multipart_error)? {
        // `field` is now an instance of `axum::extract::multipart::Field`
        // It's declared `mut` so `field.chunk()` can be called.

        if let Some(original_filename) = field.file_name() {
            let original_filename_str = original_filename.to_string();

            // Sanitize filename to prevent directory traversal and invalid characters
            let sanitized_filename = sanitize_filename::sanitize(&original_filename_str);
            if sanitized_filename.is_empty()
                || sanitized_filename == "."
                || sanitized_filename == ".."
            {
                eprintln!("Attempt to upload file with invalid sanitized name. Original: '{original_filename_str}', Sanitized: '{sanitized_filename}'");
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid filename after sanitization.".to_string(),
                ));
            }

            let path = StdPath::new("upload").join(&sanitized_filename);

            // Create file asynchronously
            let mut file = TokioFile::create(&path).await.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to create file '{sanitized_filename}': {e}"),
                )
            })?;

            // Stream chunks from the field and write to the file
            while let Some(chunk) = field.chunk().await.map_err(map_multipart_error)? {
                file.write_all(&chunk).await.map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to write chunk to file '{sanitized_filename}': {e}"),
                    )
                })?;
            }
            // Successfully wrote all chunks for this file field.
        } else {
            // This field is not a file or has no filename.
            // It's important to consume its data to allow processing of subsequent fields.
            let field_name_for_log = field.name().map(|s| s.to_string()); // Get name before consuming all chunks
            let mut total_bytes_consumed = 0;
            while let Some(chunk) = field.chunk().await.map_err(map_multipart_error)? {
                total_bytes_consumed += chunk.len();
            }

            if total_bytes_consumed > 0 {
                // You might want to use a proper logger here (e.g., tracing, log crate)
                println!(
                    "Consumed {} bytes from a non-file multipart field (name: {:?}).",
                    total_bytes_consumed,
                    field_name_for_log.as_deref().unwrap_or("unknown")
                );
            } else {
                println!(
                    "Processed a non-file multipart field with no data (name: {:?}).",
                    field_name_for_log.as_deref().unwrap_or("unknown")
                );
            }
        }
    }

    // If the loop completes, all fields were processed successfully.
    Ok(Redirect::to("/upload"))
}

