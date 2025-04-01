use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Argument {
    /// Configuration file
    #[arg(short, long, default_value_t = String::from("config.json"))]
    pub config: String,

    /// Rotation timeout in seconds
    #[arg(short, long, default_value_t = 60)]
    pub timeout: u64,

    /// Start node miner on connect
    #[arg(short, long)]
    pub mine: bool,
}
