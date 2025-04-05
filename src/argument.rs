use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Argument {
    /// Configuration file, required
    #[arg(short, long)]
    pub config: String,

    /// Rotation queue delay, seconds
    #[arg(short, long, default_value_t = 60)]
    pub delay: u64,

    /// RPC host
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    pub host: String,

    /// Processor jobs to mine at once
    #[arg(short, long, long)]
    pub jobs: Option<i64>,

    /// Miner latency in seconds, useful when blocks are being generated too quickly
    #[arg(short, long)]
    pub latency: Option<u64>,

    /// Rotation mode:
    /// * `c` - cycle
    /// * `s` - stop, disable worker
    #[arg(short, long, default_value_t = String::from("c"))]
    pub mode: String,

    /// RPC password
    #[arg(short, long, default_value_t = String::from("pwd"))]
    pub password: String,

    /// RPC port
    #[arg(long, default_value_t = 28332)]
    pub port: u16,

    /// Rotations quantity, before apply rotation `mode`
    #[arg(short, long)]
    pub rotations: Option<usize>,

    /// RPC scheme
    #[arg(short, long, default_value_t = String::from("http"))]
    pub scheme: String,

    /// RPC user
    #[arg(short, long, default_value_t = String::from("user"))]
    pub user: String,

    /// Wait to server reconnect, seconds
    #[arg(short, long, default_value_t = 900)]
    pub wait: u64,
}
