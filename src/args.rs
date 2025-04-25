use anyhow::Result;
use clap::{Parser, Subcommand, ValueEnum};
use futures::StreamExt;
use log::info;
use std::{collections::HashMap, env};
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterAccounts,
};

type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

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
        let endpoint = match &self.endpoint {
            Some(endpoint) => endpoint.clone(),
            None => env::var("GRPC_ENDPOINT").unwrap(),
        };
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
    Subscribe(Box<ActionSubscribe>),
    Ping {
        #[clap(long, short, default_value_t = 0)]
        count: i32,
    },
    GetLatestBlockhash,
    GetBlockHeight,
    GetSlot,
    IsBlockhashValid {
        #[clap(long, short)]
        blockhash: String,
    },
}
impl Action {
    async fn get_subscribe_requrest(
        &self,
        commitment: Option<CommitmentLevel>,
    ) -> Result<Option<SubscribeRequest>> {
        let result = match self {
            Action::Subscribe(args) => {
                let mut accounts: AccountFilterMap = AccountFilterMap::default();
                unimplemented!()
            }
            _ => None,
        };
        Ok(result)
    }
}

#[derive(Debug, Clone, clap::Args)]
struct ActionSubscribe {
    /// Subscribe on accounts updates
    #[clap(long)]
    accounts: bool,
    /// Filter by presence of field txn_signature
    accounts_nonempty_txn_signature: Option<bool>,
    /// Filter by Account Pubkey
    #[clap(long)]
    accounts_account: Vec<String>,
    /// Path to a JSON array of account addresses
    #[clap(long)]
    accounts_account_path: Option<String>,
    /// Filter by Owner Pubkey
    #[clap(long)]
    accounts_owner: Vec<String>,
    /// Filter by Offset and Data, format: `offset,data in base58`
    #[clap(long)]
    accounts_memcmp: Vec<String>,
    /// Filter by Data size
    #[clap(long)]
    accounts_datasize: Option<u64>,
    /// Filter valid token accounts
    #[clap(long)]
    accounts_token_account_state: bool,

    /// Filter by lamports, format: `eq:42` / `ne:42` / `lt:42` / `gt:42`
    #[clap(long)]
    accounts_lamports: Vec<String>,
    /// Receive only part of updated data account, format: `offset,size`
    #[clap(long)]
    accounts_data_slice: Vec<String>,
    /// Subscribe on slots updates
    #[clap(long)]
    slots: bool,
}

pub fn pretty_print_json(input: &str, prefix: &str) -> Result<()> {
    let s: serde_json::Value = serde_json::from_str(input)?;
    println!("{}: {}", prefix, serde_json::to_string_pretty(&s)?);
    Ok(())
}

pub async fn greyser_health_watch(client: &mut GeyserGrpcClient<impl Interceptor>) -> Result<()> {
    let mut stream = client.health_watch().await?;
    info!("health_watch stream started...");
    while let Some(health) = stream.next().await {
        info!("health message: {:?}", health);
    }
    info!("health_watch stream ended...");
    Ok(())
}
