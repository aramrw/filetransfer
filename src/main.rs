mod router;
mod dl;
mod upload;
use router::APP;
use std::{fs, net::SocketAddr, path::Path};

#[tokio::main]
async fn main() {
    if !Path::new("./upload").exists() {
        fs::create_dir_all("./upload").expect("Failed to create upload directory");
    }
    //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let tcpl = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(
        tcpl,
        APP.clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
