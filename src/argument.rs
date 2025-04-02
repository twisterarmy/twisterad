use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Argument {
    /// Configuration file
    #[arg(short, long, default_value_t = String::from("config.json"))]
    pub config: String,

    /// Rotation queue delay, seconds
    #[arg(short, long, default_value_t = 60)]
    pub delay: u64,

    /// Processors limit to mine
    #[arg(short, long)]
    pub processors: Option<i64>,

    /// Rotate ads or stop the miner on complete
    #[arg(short, long)]
    pub rotate: bool,

    /// Wait to server reconnect, seconds
    #[arg(short, long, default_value_t = 900)]
    pub wait: u64,
}
