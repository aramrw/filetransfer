use axum::{extract::DefaultBodyLimit, response::Html, routing::get, Router};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use std::sync::LazyLock;

use crate::{
    dl::{download_file, html_dl},
    upload::{html_upload, upload_file},
};

pub(crate) static APP: LazyLock<Router> = LazyLock::new(|| {
    Router::new()
        .route("/", get(root))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .route("/upload", get(html_upload).post(upload_file))
        .layer(DefaultBodyLimit::max(1_073_741_824))
        .route("/download", get(html_dl))
        .route("/download/file/{filename}", get(download_file))
});

async fn root(insecure_ip: InsecureClientIp, secure_ip: SecureClientIp) -> Html<&'static str> {
    println!("[+] conn: {insecure_ip:?} | {secure_ip:?}");
    Html(
        "
        <a href=\"/download\">Download Files</a>
        <a href=\"/upload\">Upload Files</a>
        ",
    )
}
