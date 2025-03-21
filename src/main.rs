mod cli;
mod dl;
mod router;
mod upload;
use cli::PDATA;
use router::APP;
use std::{fs, net::SocketAddr, path::Path};

#[tokio::main]
async fn main() {
    strace::dbug!(*PDATA, "serving");
    if !Path::new("./upload").exists() {
        fs::create_dir_all("./upload").expect("Failed to create upload directory");
    }
    let tcpl = tokio::net::TcpListener::bind(PDATA.cli.addr).await.unwrap();
    axum::serve(
        tcpl,
        APP.clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
