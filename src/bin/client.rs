use backoff::{ExponentialBackoff, future::retry};
use clap::Parser;
use dotenv::dotenv;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use yellowstone_grpc_command::args::{Action, Args, greyser_health_watch, pretty_print_json};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists
    dotenv().ok();
    env_logger::init();

    let args = Args::parse();
    let zero_attempts = Arc::new(Mutex::new(true));
    // The default exponential backoff strategy intervals:
    // [500ms, 750ms, 1.125s, 1.6875s, 2.53125s, 3.796875s, 5.6953125s,
    // 8.5s, 12.8s, 19.2s, 28.8s, 43.2s, 64.8s, 97s, ... ]
    retry(ExponentialBackoff::default(), move || {
        let args = args.clone();
        let zero_attempts = Arc::clone(&zero_attempts);
        async move {
            let mut zero_attempts = zero_attempts.lock().await;
            if *zero_attempts {
                *zero_attempts = false;
            } else {
                info!("Retry to connect to the server");
            }
            drop(zero_attempts);
            let commitment = args.get_commitment();
            let mut client = args.connect().await.map_err(backoff::Error::transient)?;
            info!("Connected");
            match &args.action {
                Action::GetVersion => client
                    .get_version()
                    .await
                    .map_err(anyhow::Error::from)
                    .and_then(|response| pretty_print_json(&response.version, "Version response")),
                Action::Ping { count } => client
                    .ping(*count)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("Ping response: {response:#?}")),
                Action::HealthWatch => greyser_health_watch(&mut client).await,
                Action::HealthCheck => client
                    .health_check()
                    .await
                    .map_err(anyhow::Error::from)
                    .map(|response| info!("Health check response: {response:?}")),
                Action::GetLatestBlockhash => client
                    .get_latest_blockhash(commitment)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("latest_blockhash: {response:#?}")),
                Action::GetSlot => client
                    .get_slot(commitment)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("slot: {response:#?}")),
                Action::GetBlockHeight => client
                    .get_block_height(commitment)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("block_height: {response:#?}")),

                Action::IsBlockhashValid { blockhash } => client
                    .is_blockhash_valid(blockhash.clone(), commitment)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("response: {response:?}")),
                // _ => {
                //     unimplemented!()
                // }
            }
            .map_err(backoff::Error::transient)?;
            Ok::<(), backoff::Error<anyhow::Error>>(())
        }
    })
    .await?;
    Ok(())
}
