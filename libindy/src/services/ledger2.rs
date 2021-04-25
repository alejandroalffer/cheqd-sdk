//! Ledger service for Cosmos back-end

use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::tx::{Msg, MsgType};
use cosmos_sdk::Coin;
use indy_api_types::errors::IndyResult;

pub struct Ledger2Service {}

impl Ledger2Service {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build_msg_bank_send(
        &self,
        sender_account_id: &str,
        recipient_account_id: &str,
        amount: u64,
        denom: &str,
    ) -> IndyResult<Msg> {
        let amount = Coin {
            amount: amount.into(),
            denom: denom.parse()?,
        };

        let msg_send = MsgSend {
            from_address: sender_account_id.parse()?,
            to_address: recipient_account_id.parse()?,
            amount: vec![amount.clone()],
        };

        Ok(msg_send.to_msg()?)
    }
}

#[cfg(test)]
mod test {
    use crate::services::{KeysService, Ledger2Service, Pool2Service};
    use cosmos_sdk::Coin;

    #[async_std::test]
    async fn test() {
        let ledger2_service = Ledger2Service::new();
        let pool2_service = Pool2Service::new();
        let keys_service = KeysService::new();

        let alice = keys_service.add_random("alice").await.unwrap();
        let bob = keys_service.add_random("bob").await.unwrap();

        let msg = ledger2_service
            .build_msg_bank_send(&alice.account_id, &bob.account_id, 1000, "stake")
            .unwrap();

        let tx = pool2_service
            .build_tx(
                &alice.pub_key,
                vec![msg],
                "mainnet",
                0,
                0,
                1000,
                &Coin {
                    amount: 1000u64.into(),
                    denom: "stake".parse().unwrap(),
                },
                10000,
                "memo",
            )
            .unwrap();

        let _signed = keys_service.sign("alice", tx).await.unwrap();

        // Broadcast

        assert!(true)
    }
}
