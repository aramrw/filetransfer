use clap::{command, Parser};
use core::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use local_ip_address::local_ip;
use std::{env, fmt::Display, path::PathBuf, sync::LazyLock};
use strace::color_print::cwrite;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(required = false, long, default_value_t = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), 3000)))]
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
            ..
        } = self;
        let exe = exe.display();
        cwrite!(
            f,
            "\n-* filetransfer <b>v{version}</b> *-\n-----------------------------\n<u><b>{repo}</b></u>\n{exe}\n<i>PID:{pid}</i>\n",
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
        let local_ip = local_ip().unwrap().to_canonical();
        let port = self.cli.addr.port();
        cwrite!(
            f,
            "{cargo}-----------------------------\n<b><u>http://{local_ip}:{port}</u><b>\n-----------------------------"
        )
    }
}
