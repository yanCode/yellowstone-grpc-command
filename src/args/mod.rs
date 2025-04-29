// Add more module declarations here as needed

mod subscribe_account;
mod subscribe_token_price;
mod subscribe_tx;
mod utils;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long)]
    pub endpoint: Option<String>,

    #[clap(short, long, default_value = "processed", value_parser = ["processed", "confirmed", "finalized"])]
    pub commitment_level: String,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Commands {
    ServerVersion,
    HealthCheck,
    LatestBlockhash,
    SubscribeTx,
    SubscribeTokenPrice,
    SubscribeAccount,
}
