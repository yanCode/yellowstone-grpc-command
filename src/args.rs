use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use std::env;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::CommitmentLevel;

#[derive(Debug, Clone, Parser)]
#[clap(
    author = "todo",
    version = "0.1.0",
    about = "command line tool for Yellowstone gRPC"
)]
pub struct Args {
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
    pub action: Action,
}
impl Args {
    pub fn get_commitment(&self) -> Option<CommitmentLevel> {
        self.commitment.map(|c| c.into())
    }

    pub async fn connect(&self) -> Result<GeyserGrpcClient<impl Interceptor>> {
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

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum ArgsCommitment {
    #[default]
    Processed,
    Confirmed,
    Finalized,
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
pub enum Action {
    GetVersion,
    HealthCheck,
    HealthWatch,
    Ping {
        #[clap(long, short, default_value_t = 0)]
        count: i32,
    },
}
