mod cli;
mod dl;
mod router;
mod upload;
use cli::PDATA;
use local_ip_address::local_ip;
use router::APP;
use std::{
    fs,
    net::{Ipv4Addr, SocketAddr},
    path::Path,
};

#[tokio::main]
async fn main() {
    println!("{}", *PDATA);
    if !Path::new("./upload").exists() {
        fs::create_dir_all("./upload").expect("Failed to create upload directory");
    }
    //let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let tcpl = tokio::net::TcpListener::bind(PDATA.cli.addr).await.unwrap();
    axum::serve(
        tcpl,
        APP.clone()
            .into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
