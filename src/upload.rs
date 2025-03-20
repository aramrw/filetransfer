use axum::{
    extract::Multipart,
    response::{Html, Redirect},
};
use std::{fs::File, io::Write, path::Path as StdPath};

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
    mut multipart: Multipart,
) -> Result<Redirect, (axum::http::StatusCode, String)> {
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let path = StdPath::new("upload").join(&file_name);
        let mut file = File::create(path).unwrap();
        while let Some(chunk) = field.chunk().await.unwrap() {
            file.write_all(&chunk).unwrap();
        }
    }
    Ok(Redirect::to("/upload"))
}
