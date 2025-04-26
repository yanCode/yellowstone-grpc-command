// Add more module declarations here as needed

mod utils;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub endpoint: Option<String>,

    #[clap(short, long, default_value = "processed")]
    pub commitment: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ServerVersion,
    HealthCheck,
    LatestBlockhash,
}
