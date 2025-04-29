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
        Commands::SubscribeTx(subscribe_tx_args) => {
            args.subscribe_tx(subscribe_tx_args.accounts.clone()).await?
        }
        Commands::SubscribeTokenPrice(subscribe_token_price_args) => {
            args.subscribe_token_price(subscribe_token_price_args.account.clone())
                .await?
        }
        Commands::SubscribeAccount(subscribe_account_args) => {
            args.subscribe_account(subscribe_account_args.account.clone())
                .await?
        }
    }
    Ok(())
}
