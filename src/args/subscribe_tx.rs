use std::collections::HashMap;

use anyhow::Result;
use chrono::Local;
use futures::sink::SinkExt;
use log::{debug, error, info};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use yellowstone_grpc_proto::geyser::{
    CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions, SubscribeRequestPing,
    subscribe_update::UpdateOneof,
};

use crate::helper::TransactionPretty;

use super::Args;

impl Args {
    pub async fn subscribe_tx(&self, account_include: Vec<String>) -> Result<()> {
        debug!(
            "Subscribing to transactions for accounts: {:?}",
            &account_include
        );

        let transactions: HashMap<String, SubscribeRequestFilterTransactions> = HashMap::from([(
            "client".to_string(),
            SubscribeRequestFilterTransactions {
                account_include,
                ..Default::default()
            },
        )]);

        let mut client = self.clone().connect().await?;

        let suscribe_request = SubscribeRequest {
            transactions,
            commitment: Some(CommitmentLevel::Processed.into()),
            ..Default::default()
        };

        let (mut subscribe_tx, mut stream) = client
            .subscribe_with_request(Some(suscribe_request))
            .await?;
        let (tx, mut rx) = mpsc::channel::<TransactionPretty>(100);

        debug!("Subscribed to transactions successfully");

        tokio::spawn(async move {
            while let Some(message) = stream.next().await {
                if let Ok(msg) = message {
                    match msg.update_oneof {
                        Some(UpdateOneof::Transaction(sut)) => {
                            let transaction_pretty: TransactionPretty = sut.into();
                            tx.try_send(transaction_pretty)?;
                        }
                        Some(UpdateOneof::Ping(_)) => {
                            subscribe_tx
                                .send(SubscribeRequest {
                                    ping: Some(SubscribeRequestPing { id: 1 }),
                                    ..Default::default()
                                })
                                .await?
                        }
                        Some(UpdateOneof::Pong(pong)) => {
                            info!("service is pong: {}", Local::now());
                            debug!("Pong: {:?}", pong);
                        }
                        _ => {}
                    }
                } else {
                    error!("Error: {:?}", message);
                    break;
                }
            }
            Ok::<(), anyhow::Error>(())
        });
        while let Some(event) = rx.recv().await {
            info!("TransactionPretty: {:?}", event);
        }

        Ok(())
    }
}
