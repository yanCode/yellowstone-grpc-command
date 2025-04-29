mod args;
mod helper;

use anyhow::Result;
use args::{Args, Commands};
use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().expect(".env file must be present!");
    env_logger::init();
    let args = Args::parse();
    let mut client = args.clone().connect().await?;

    match &args.command {
        Commands::ServerVersion => args.server_version(&mut client).await?,
        Commands::HealthCheck => args.greyser_health_watch(&mut client).await?,
        Commands::LatestBlockhash => args.get_latest_blockhash(&mut client).await?,
        Commands::SubscribeTx => args.subscribe_tx().await?,
        Commands::SubscribeTokenPrice => args.subscribe_token_price().await?,
        Commands::SubscribeAccount => args.subscribe_account().await?,
    }
    Ok(())
}
