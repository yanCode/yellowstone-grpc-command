use anyhow::Result;
use backoff::{ExponentialBackoff, future::retry};
use clap::{Parser, Subcommand, ValueEnum};
use dotenv::dotenv;
use log::{error, info};
use std::{env, sync::Arc};
use tokio::sync::Mutex;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::CommitmentLevel;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load .env file if it exists
    dotenv().ok();
    env_logger::init();

    let args = Args::parse();
    println!("args: {:?}", args);
    // let zero_attempts = Arc::new(Mutex::new(true));
    // The default exponential backoff strategy intervals:
    // [500ms, 750ms, 1.125s, 1.6875s, 2.53125s, 3.796875s, 5.6953125s,
    // 8.5s, 12.8s, 19.2s, 28.8s, 43.2s, 64.8s, 97s, ... ]
    // retry(ExponentialBackoff::default(), move || {
    //     let args = args.clone();
    //     let zero_attempts = Arc::clone(&zero_attempts);
    //     async move || Ok(())
    // })
    // .await;
    let _commitment = args.get_commitment();
    let mut client = args.connect().await?;
    info!("Connected...");
    match &args.action {
        Action::GetVersion => {
            let version = client.get_version().await?;
            info!("Version: {}", version.version);
        }
        Action::HealthCheck => {
            let health = client.health_check().await?;
            info!("Health: {:?}", health);
        }
        _ => {
            unimplemented!()
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum ArgsCommitment {
    #[default]
    Processed,
    Confirmed,
    Finalized,
}

#[derive(Debug, Clone, Parser)]
#[clap(
    author = "todo",
    version = "0.1.0",
    about = "command line tool for Yellowstone gRPC"
)]
struct Args {
    #[clap(short, long)]
    /// Service endpoint
    endpoint: Option<String>,
    /// Apply a timeout to connecting to the uri.
    #[clap(long)]
    connect_timeout_ms: Option<u64>,
    /// Commitment level: processed, confirmed or finalized
    #[clap(long)]
    commitment: Option<ArgsCommitment>,
    #[clap(subcommand)]
    action: Action,
}
impl Args {
    fn get_commitment(&self) -> Option<CommitmentLevel> {
        self.commitment.map(|c| c.into())
    }

    async fn connect(&self) -> Result<GeyserGrpcClient<impl Interceptor>> {
        let tls_config = ClientTlsConfig::new().with_native_roots();
        let endpoint = self
            .endpoint
            .clone()
            .unwrap_or(env::var("GRPC_ENDPOINT").unwrap());
        println!("endpoint: {}", endpoint);
        let builder = GeyserGrpcClient::build_from_shared(endpoint)?.tls_config(tls_config)?;

        builder
            .connect()
            .await
            .map_err(|e| anyhow::anyhow!("failed to connect: {}", e))
    }
}
impl From<ArgsCommitment> for CommitmentLevel {
    fn from(commitment: ArgsCommitment) -> Self {
        match commitment {
            ArgsCommitment::Processed => CommitmentLevel::Processed,
            ArgsCommitment::Confirmed => CommitmentLevel::Confirmed,
            ArgsCommitment::Finalized => CommitmentLevel::Finalized,
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
enum Action {
    GetVersion,
    HealthCheck,
    HealthWatch,
    Ping {
        #[clap(long, short, default_value_t = 0)]
        count: i32,
    },
}
