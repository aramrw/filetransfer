use axum::{
    extract::{DefaultBodyLimit, Request},
    http::StatusCode,
    middleware::{self, Next},
    response::Html,
    response::Response,
    routing::get,
    Router,
};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use std::{
    //net::{SocketAddr, SocketAddrV4},
    sync::LazyLock,
};

use crate::{
    cli::PDATA,
    dl::{download_file, html_dl},
    upload::{html_upload, upload_file},
};

async fn check_client_ip(req: Request, next: Next) -> Result<Response, StatusCode> {
    let ip = req
        .extensions()
        .get::<SecureClientIp>()
        .ok_or(StatusCode::FORBIDDEN)?
        .0;

    if ip != PDATA.cli.addr.ip() {
        println!("non-whitelist ip attempted to connect: {ip}");
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}

pub(crate) static APP: LazyLock<Router> = LazyLock::new(|| {
    Router::new()
        .layer(middleware::from_fn(check_client_ip))
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
