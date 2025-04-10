use backoff::{ExponentialBackoff, future::retry};
use clap::Parser;
use dotenv::dotenv;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use yellowstone_grpc_command::args::{Action, Args};

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
            let _commitment = args.get_commitment();
            let mut client = args.connect().await.map_err(backoff::Error::transient)?;
            info!("Connected");
            match &args.action {
                Action::GetVersion => client
                    .get_version()
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("version response: {response:?}")),
                Action::Ping { count } => client
                    .ping(*count)
                    .await
                    .map_err(anyhow::Error::new)
                    .map(|response| info!("ping response: {response:?}")),
                _ => {
                    unimplemented!()
                }
            }
            .map_err(backoff::Error::transient)?;
            Ok::<(), backoff::Error<anyhow::Error>>(())
        }
    })
    .await?;
    Ok(())
}
