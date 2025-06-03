use clap::{command, Parser};
use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use local_ip_address::local_ip; // Keep this
use std::{env, fmt::Display, path::PathBuf, sync::LazyLock};
use strace::color_print::cwrite; // Assuming strace is your crate

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(required = false, long, default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0,0,0,0), 3000)))]
    pub addr: SocketAddr,
}

pub struct CargoInfo {
    version: &'static str,
    repo: &'static str,
    exe: PathBuf,
    pid: u32,
}

impl Display for CargoInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CargoInfo {
            version,
            repo,
            exe,
            pid,
            .. // Removed trailing comma if it was there, it's fine though
        } = self;
        let exe_display = exe.display(); // Renamed to avoid conflict, and use it
        cwrite!(
            f,
            "\n-* filetransfer <b>v{version}</b> *-\n-----------------------------\n<u><b>{repo}</b></u>\n\
            {exe_display}\n<i>PID:{pid}</i>\n", // Used exe_display
        )
    }
}

impl Default for CargoInfo {
    fn default() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
            repo: env!("CARGO_PKG_REPOSITORY"),
            exe: env::current_exe().expect("fatal: failed to locate current executable path"),
            pid: std::process::id(),
        }
    }
}

pub struct ProgramData {
    pub cargo: CargoInfo,
    pub cli: Cli,
}

pub static PDATA: LazyLock<ProgramData> = LazyLock::new(|| ProgramData {
    cargo: CargoInfo::default(),
    cli: Cli::parse(),
});

impl Display for ProgramData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ProgramData { cargo, .. } = self;
        
        // Handle potential error when fetching local IP
        let local_ip_str = match local_ip() {
            Ok(ip) => ip.to_string(), // Using to_string for simpler display, canonical is fine too
            Err(_) => "[Could not determine local IP]".to_string(),
        };
        
        let port = self.cli.addr.port();
        let server_addr_ip_str = if self.cli.addr.ip().is_unspecified() {
            // If listening on 0.0.0.0 or ::, use the determined local_ip_str for display,
            // or show 0.0.0.0 to indicate it's accessible on all interfaces.
            // For simplicity, using local_ip_str here for user-friendly URL.
            local_ip_str
        } else {
            self.cli.addr.ip().to_string()
        };

        cwrite!(
            f,
            "{cargo}-----------------------------\n<b><u>http://{server_addr_ip_str}:{port}</u></b>\n----------------\
            -------------"
        )
    }
}
