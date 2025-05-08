use std::fmt::{self, Debug, Display, Formatter};

use solana_sdk::signature::Signature;
use solana_transaction_status::{
    EncodedTransactionWithStatusMeta, TransactionWithStatusMeta, UiTransactionEncoding,
};
use yellowstone_grpc_proto::geyser::SubscribeUpdateTransaction;

pub struct TransactionPretty {
    pub slot: u64,
    pub signature: Signature,
    pub is_vote: bool,
    pub tx: TransactionWithStatusMeta,
}

impl Debug for TransactionPretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        struct TxWrap<'a>(&'a EncodedTransactionWithStatusMeta);
        impl<'a> Debug for TxWrap<'a> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                let serialized = serde_json::to_string_pretty(self.0).unwrap();
                Display::fmt(&serialized, f)
            }
        }
        f.debug_struct("TransactionPretty")
            .field("slot", &self.slot)
            .field("signature", &self.signature)
            .field("is_vote", &self.is_vote)
            .field(
                "tx",
                &TxWrap(
                    &self
                        .tx
                        .clone()
                        .encode(UiTransactionEncoding::Base64, Some(u8::MAX), false)
                        .expect("failed to encode TransactionWithStatusMeta"),
                ),
            )
            .finish()
    }
}

impl From<SubscribeUpdateTransaction> for TransactionPretty {
    fn from(SubscribeUpdateTransaction { transaction, slot }: SubscribeUpdateTransaction) -> Self {
        let tx = transaction.expect("should be defined");
        Self {
            slot,
            signature: Signature::try_from(tx.signature.as_slice()).expect("valid signature"),
            is_vote: tx.is_vote,
            tx: yellowstone_grpc_proto::convert_from::create_tx_with_meta(tx)
                .expect("valid tx with meta"),
        }
    }
}
