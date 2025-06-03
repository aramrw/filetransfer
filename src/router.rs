use axum::{
    extract::{Request, DefaultBodyLimit}, // DefaultBodyLimit might be here or on handler
    http::StatusCode,
    middleware::{self, Next},
    response::Html, // Assuming Html is used by root
    response::Response,
    routing::get,
    Router,
    handler::Handler, // If .layer is on handler
};
use axum_client_ip::{InsecureClientIp, SecureClientIp, SecureClientIpSource};
use std::{net::IpAddr, sync::LazyLock}; // Added IpAddr
use strace::color_print::cprintln; // Your logging utility

// PDATA import is likely not needed here if check_client_ip doesn't use it.
// use crate::cli::PDATA; 
use crate::{
    // cli::PDATA, // Remove if not used in this file after changes
    dl::{download_file, html_dl},
    upload::{html_upload, upload_file},
    // root handler might be defined in this file or imported
};


// Helper function to determine if an IP is typically local or private.
fn is_local_or_private_ip(ip: IpAddr) -> bool {
    if ip.is_loopback() {
        return true;
    }
    match ip {
        IpAddr::V4(ipv4) => {
            let octets = ipv4.octets();
            match octets[0] {
                10 => true, // 10.0.0.0/8
                172 => (octets[1] >= 16 && octets[1] <= 31), // 172.16.0.0/12
                192 => octets[1] == 168, // 192.168.0.0/16
                169 => octets[1] == 254, // 169.254.0.0/16 (Link-local/APIPA)
                _ => false,
            }
        }
        IpAddr::V6(ipv6) => {
            // Check for Unique Local Addresses (fc00::/7) or Link-Local Addresses (fe80::/10)
            (ipv6.segments()[0] & 0xfe00 == 0xfc00) || (ipv6.segments()[0] & 0xffc0 == 0xfe80)
        }
    }
}

async fn check_client_ip(
    insecure_ip: InsecureClientIp,
    secure_ip: SecureClientIp,
    req: Request, // Keep if you want to log method/uri from req
    next: Next,
) -> Result<Response, StatusCode> {
    let client_ip = secure_ip.0;

    // Your existing logging for the connection attempt
    cprintln!(
        "<g>[</g>+<g>]</g> \\\\(<r>{:?}</r> / <c>{:?}</c>\\\\) % <b>{:?}</b> {:?}",
        insecure_ip.0,
        client_ip, // Using secure_ip.0 which is client_ip here
        req.method(),
        req.uri()
    );

    if is_local_or_private_ip(client_ip) {
        // Optionally, you could add a log message here for allowed connections if desired
        // cprintln!("<g>INFO</g> Allowed local/private IP: {}", client_ip);
        Ok(next.run(req).await)
    } else {
        println!(
            "Connection attempt from public IP {} denied. Only local/private network access is allowed.",
            client_ip
        );
        Err(StatusCode::FORBIDDEN)
    }
}

// Your APP static LazyLock definition remains here...
// Ensure PDATA is not imported if it's no longer used in this file.
// For example:
pub(crate) static APP: LazyLock<Router> = LazyLock::new(|| {
    const MAX_UPLOAD_SIZE: usize = 53_687_091_200; // 50 GiB

    Router::new()
        .layer(middleware::from_fn(check_client_ip))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
        .route("/", get(root)) // Assuming root is defined or imported
        .route(
            "/upload",
            get(html_upload).post(
                upload_file.layer(DefaultBodyLimit::max(MAX_UPLOAD_SIZE))
            )
        )
        .route("/download", get(html_dl))
        .route("/download/file/{filename}", get(download_file)) // Corrected path param
});

// Your original root function restored
async fn root() -> Html<&'static str> {
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
                    /* display: flex; */ /* 'display: flex' is duplicated, inline-block is usually sufficient for button-like links unless content within 'a' needs flex alignment */
                    justify-content: center; /* Effective if display is flex */
                    align-items: center; /* Effective if display is flex */
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
                    gap: 3px; /* Consider increasing for better touch targets, e.g., 10px */
                    padding: 0; /* Good for ul acting as a link container */
                    list-style-type: none; /* Good for ul acting as a link container */
                }
                .github-icon {
                    border-radius: 4px;
                    display: block; /* Or inline-block if you want it to flow differently */
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
            </ul> </body>
        </html>
        "#,
    )
}
