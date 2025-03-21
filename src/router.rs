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
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>File Manager</title>
            <style>
                body {
                    font-family: Arial, sans-serif;
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    height: 100vh;
                    background-color: #f4f4f4;
                    margin: 0;
                }
                .container {
                    display: flex;
                    flex-direction: column;
                    text-align: center;
                    background: white;
                    padding: 20px;
                    border-radius: 10px;
                    box-shadow: 0 0 10px rgba(0, 0, 0, 0.1);
                }
                a {
                    display: inline-block;
                    padding: 10px 15px;
                    text-decoration: none;
                    display: flex;
                    justify-content: center; 
                    align-items: center;
                    color: white;
                    background-color: #007bff;
                    border-radius: 5px;
                    transition: background-color 0.3s;
                }
                a:hover {
                    background-color: #0056b3;
                }
                .root_links {
                    display: flex;
                    flex-direction: column;
                    justify-content: center; 
                    align-items: center;
                    gap: 3px;
                }
                .github-icon {
                    border-radius: 4px;
                    display: block;
                    width: 30px;
                    height: 30px;
                }
            </style>
        </head>
        <body>
                <h1>File Transfer</h1>
                <ul class="root_links">
                    <a href="/download">Download Files</a>
                    <a href="/upload">Upload Files</a>
                    <a href="https://github.com/aramrw/filetransfer">
                        <img 
                            class="github-icon" 
                            src="https://github.githubassets.com/images/modules/logos_page/GitHub-Mark.png" 
                            alt="GitHub"
                        />
                    </a>
                <ul>
        </body>
        </html>
        "#,
    )
}
