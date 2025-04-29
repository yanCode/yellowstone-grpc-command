use std::collections::HashMap;

use anyhow::Result;
use futures::SinkExt;
use log::{debug, error, info};
use solana_sdk::{program_pack::Pack, pubkey::Pubkey, signature::Signature};
use tokio_stream::StreamExt;
use yellowstone_grpc_proto::geyser::{
    SubscribeRequest, SubscribeRequestFilterAccounts, SubscribeRequestPing, SubscribeUpdateAccount,
    subscribe_update::UpdateOneof,
};

use super::Args;

impl Args {
    pub async fn subscribe_account(&self) -> Result<()> {
        let addrs = vec!["53gas6nwoz3GjbdbmiZ5ywLdfnhqUQNEKvF5ErpXUc3S".to_string()];
        let mut client = self.clone().connect().await?;
        let subscribe_request = SubscribeRequest {
            accounts: HashMap::from([(
                "client".to_string(),
                SubscribeRequestFilterAccounts {
                    account: addrs,
                    ..Default::default()
                },
            )]),
            commitment: Some(self.get_commitment_level().into()),
            ..Default::default()
        };

        let (mut sbuscribe_tx, mut stream) = client
            .subscribe_with_request(Some(subscribe_request))
            .await?;
        while let Some(message) = stream.next().await {
            match message {
                Ok(update) => match update.update_oneof {
                    Some(UpdateOneof::Account(account)) => {
                        parse_account(&account)?;
                    }
                    Some(UpdateOneof::Ping(ping)) => {
                        debug!("service is pinging..");
                        sbuscribe_tx
                            .send(SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: 1 }),
                                ..Default::default()
                            })
                            .await?;
                    }
                    Some(UpdateOneof::Pong(pong)) => {
                        info!("pong: {:?}", pong);
                    }
                    _ => {
                        unimplemented!()
                    }
                },
                Err(e) => {
                    error!("error: {:?}", e);
                }
            }
        }
        Ok(())
    }
}

fn parse_account(subscribe_account: &SubscribeUpdateAccount) -> Result<()> {
    if let Some(account) = &subscribe_account.account {
        info!("account: {:?}", account);
        let account_pubkey = Pubkey::try_from(account.pubkey.as_slice())?;
        info!("account_pubkey: {:#?}", account_pubkey);
        let owner = Pubkey::try_from(account.owner.as_slice())?;
        info!("owner: {:#?}", owner);
        let account_signture = match Signature::try_from(account.txn_signature()) {
            Ok(signature) => signature,
            Err(e) => {
                info!("error: {:?}", e);
                return Err(e.into());
            }
        };
        let account_info = match spl_token::state::Account::unpack(&account.data) {
            Ok(info) => info,
            Err(e) => {
                error!("account_info error: {:#?}", e);
                return Err(e.into());
            }
        };
        info!("account_signture: {:#?}", account_signture);
        info!("account_info: {:#?}", account_info);
    }

    Ok(())
}
