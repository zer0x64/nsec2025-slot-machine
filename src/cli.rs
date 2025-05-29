use clap::Parser;
use std::net::SocketAddr;

/// Slot machine server
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Bind address to listen to
    #[arg(default_value = "127.0.0.1:8000")]
    #[arg(short, long)]
    pub bind: SocketAddr,
}
