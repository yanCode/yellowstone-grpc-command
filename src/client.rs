use anyhow::Ok;
use backoff::{ExponentialBackoff, future::retry};
use clap::{Parser, ValueEnum};
use dotenv::dotenv;
use std::{env, sync::Arc};
use tokio::sync::Mutex;
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
    let commitment = args.get_commitment();
    println!("commitment: {:?}", commitment);
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
    author = "yuki@yuki0327.com",
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
}
impl Args {
    fn get_commitment(&self) -> Option<CommitmentLevel> {
        self.commitment.map(|c| c.into())
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
