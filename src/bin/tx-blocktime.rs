use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Utc};
use clap::Parser;
use futures::SinkExt;
use log::info;
use maplit::hashmap;
use solana_sdk::signature::Signature;
use tokio_stream::StreamExt;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient};
use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterTransactions, subscribe_update::UpdateOneof,
};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, default_value_t = String::from("http://127.0.0.1:10000"))]
    /// Service endpoint
    endpoint: String,

    #[clap(long)]
    x_token: Option<String>,

    /// Commitment level: processed, confirmed or finalized
    // #[clap(long)]//todo: add commitment level
    // commitment: Option<ArgsCommitment>,

    /// Filter vote transactions
    #[clap(long)]
    vote: Option<bool>,

    /// Filter failed transactions
    #[clap(long)]
    failed: Option<bool>,

    /// Filter by transaction signature
    #[clap(long)]
    signature: Option<String>,

    /// Filter included account in transactions
    #[clap(long)]
    account_include: Vec<String>,

    /// Filter excluded account in transactions
    #[clap(long)]
    account_exclude: Vec<String>,

    /// Filter required account in transactions
    #[clap(long)]
    account_required: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let args = Args::parse();
    let mut client = GeyserGrpcClient::build_from_shared(args.endpoint)?
        .x_token(args.x_token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    let commitment: CommitmentLevel = CommitmentLevel::Processed;

    subscribe_tx
        .send(SubscribeRequest {
            //todo why send has sink trait?
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            transactions_status: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: args.vote,
                failed: args.failed,
                signature: args.signature,
                account_include: args.account_include,
                account_exclude: args.account_exclude,
                account_required: args.account_required,
            } },
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: hashmap! { "".to_owned() => SubscribeRequestFilterBlocksMeta {} },
            commitment: Some(commitment as i32),
            accounts_data_slice: vec![],
            ping: None,
            from_slot: None,
        })
        .await?;
    let mut messages: BTreeMap<u64, (Option<DateTime<Utc>>, Vec<String>)> = BTreeMap::new();
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                match msg.update_oneof {
                    Some(UpdateOneof::TransactionStatus(tx)) => {
                        let entry = messages.entry(tx.slot).or_default();
                        let sig = Signature::try_from(tx.signature.as_slice())
                            .expect("valid signature from transaction")
                            .to_string();
                        if let Some(timestamp) = entry.0 {
                            info!("received txn {} at {}", sig, timestamp);
                        } else {
                            entry.1.push(sig);
                        }
                    }
                    Some(UpdateOneof::BlockMeta(block)) => {
                        let entry = messages.entry(block.slot).or_default();
                        entry.0 = block.block_time.map(|obj| {
                            DateTime::from_timestamp(obj.timestamp, 0)
                                .expect("invalid or out-of-range datetime")
                        });
                        if let Some(timestamp) = entry.0 {
                            for sig in &entry.1 {
                                info!("received txn {} at {}", sig, timestamp);
                            }
                        }

                        // remove outdated
                        while let Some(slot) = messages.keys().next().cloned() {
                            if slot < block.slot - 20 {
                                messages.remove(&slot);
                            } else {
                                break;
                            }
                        }
                    }
                    _ => {}
                }
            }
            Err(error) => {
                log::error!("stream error: {error:?}");
                break;
            }
        }
    }
    Ok(())
}
