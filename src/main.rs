mod args;
mod helper;
use anyhow::Result;
use args::{Args, Commands};
use clap::Parser;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();
    let args = Args::parse();
    let mut client = args.connect().await?;

    match &args.command {
        Commands::ServerVersion => args.server_version(&mut client).await?,
        Commands::HealthCheck => args.greyser_health_watch(&mut client).await?,
        Commands::LatestBlockhash => args.get_latest_blockhash(&mut client).await?,
    }
    Ok(())
}
