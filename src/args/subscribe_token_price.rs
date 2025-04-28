use std::collections::HashMap;

use crate::helper::TransactionPretty;

use super::Args;
use anyhow::Result;
use log::info;
use tokio::sync::mpsc;
use yellowstone_grpc_proto::geyser::{CommitmentLevel, SubscribeRequest, SubscribeRequestFilterAccounts};
use tokio_stream::StreamExt;

impl Args {
    pub async fn subscribe_token_price(&self) -> Result<()> {
        let mut client = self.clone().connect().await?;
        let subscribe_request = SubscribeRequest {
            accounts: HashMap::from([(
                "client".to_string(),
                SubscribeRequestFilterAccounts {
                    account: vec!["8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj".to_string()],
                    owner: vec!["8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj".to_string()],
                    ..Default::default()
                },
            )]),
            commitment: Some(CommitmentLevel::Processed as i32),
            ..Default::default()
        };
        info!("subscribe_token_price");
        let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(subscribe_request)).await?;
        let (tx, mut rx) = mpsc::channel::<TransactionPretty>(100);

        println!("Subscribed to transactions...");

        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
               info!("message: {:?}", message);
            }
        });
        
        while let Some(event) = rx.recv().await {
            info!("TransactionPretty: {:?}", event);
        }

        Ok(())
    }
}
