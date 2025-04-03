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

    /// Rotation mode:
    /// * `c` - continue in cycle
    /// * `s` - stop, disable worker
    #[arg(short, long, default_value_t = String::from("c"))]
    pub mode: String,

    /// Processors limit to mine
    #[arg(short, long)]
    pub processors: Option<i64>,

    /// Iterations quantity before apply rotation `mode`
    #[arg(short, long)]
    pub quantity: Option<usize>,

    /// Rotate ads or stop the miner on complete
    #[arg(short, long)]
    pub rotate: bool,

    /// Wait to server reconnect, seconds
    #[arg(short, long, default_value_t = 900)]
    pub wait: u64,
}
