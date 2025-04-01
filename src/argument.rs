use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Argument {
    /// Configuration file, required
    #[arg(short, long)]
    pub config: String,

    /// Rotate messages time in seconds (`60` by default)
    #[arg(short, long, default_value_t = 60)]
    pub rotate: u64,
}
