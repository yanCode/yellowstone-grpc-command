use std::collections::HashMap;

use super::Args;
use anyhow::Result;
use futures::SinkExt;
use log::{debug, info};
use tokio::sync::mpsc;
use yellowstone_grpc_proto::geyser::{subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestAccountsDataSlice, SubscribeRequestFilterAccounts, SubscribeRequestPing};
use tokio_stream::StreamExt;

impl Args {
    pub async fn subscribe_token_price(&self) -> Result<()> {
        let mut client = self.clone().connect().await?;
        let subscribe_request = SubscribeRequest {
            accounts: HashMap::from([(
                "client".to_string(),
                SubscribeRequestFilterAccounts {
                    account: vec!["8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj".to_string()],
                    owner: vec!["CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK".to_string()],
                    ..Default::default()
                },
            )]),
            accounts_data_slice: vec![SubscribeRequestAccountsDataSlice {
                offset: 253,
                length: 16,
            }],
            commitment: Some(CommitmentLevel::Processed as i32),
            ..Default::default()
        };
        info!("subscribe_token_price");
        let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(subscribe_request)).await?;
        let (tx, mut rx) = mpsc::channel(100);

        println!("Subscribed to transactions...");

        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
               match message?.update_oneof{
                Some(UpdateOneof::Account(sut))=>{
                  let data = sut.account.unwrap().data;
                  let sqrt_price_x64 = u128::from_le_bytes(data[0..16].try_into()?);
                  let sqrt_price_x64_float = sqrt_price_x64 as f64 / (1u128 << 64) as f64;
                  let price = sqrt_price_x64_float.powi(2) * 1e9 / 1e6;
               
                  tx.try_send(price)?;
                }
                Some(UpdateOneof::Pong(pong))=>{
                    debug!("service is pong: {}", pong.id);
                   
                }
                Some(UpdateOneof::Ping(_))=>{
                    debug!("service is pinging..");
                    subscribe_tx.send(SubscribeRequest{
                        ping: Some(SubscribeRequestPing{id:1}),
                        ..Default::default()
                    }).await?;
                }
                _=>{}
               }
            }
            Ok::<(), anyhow::Error>(())
        });

        while let Some(event) = rx.recv().await {
            info!("WSOL Price: {}", event);
        }

        Ok(())
    }
}
