//! Ledger service for Cosmos back-end

use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::tx::{Msg, MsgType};
use cosmos_sdk::Coin;
use indy_api_types::errors::{IndyResult, IndyErrorKind};
use indy_api_types::IndyError;

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

    pub fn build_msg_create_nym(
        &self,
        alias: &str,
        verkey: &str,
        did: &str,
        role: &str,
        from: &str,
    ) -> IndyResult<Msg> {

        let msg_send = Nym {
            creator: from.to_string(),
            id: 0,
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: role.to_string(),
        };

        Ok(msg_send.to_msg()?)
    }
}



#[cfg(test)]
mod test {
    use crate::services::{KeysService, Ledger2Service, Pool2Service};
    use cosmos_sdk::crypto::secp256k1::SigningKey;

    #[async_std::test]
    async fn test_tx_commit_flow() {
        let ledger2_service = Ledger2Service::new();
        let pool2_service = Pool2Service::new();
        let keys_service = KeysService::new();

        let alice = keys_service
            .add_from_mnemonic("alice", "alice")
            .await
            .unwrap();
        let bob = keys_service.add_from_mnemonic("bob", "bob").await.unwrap();

        println!("Alice's account id: {}", alice.account_id);
        println!("Bob's account id: {}", bob.account_id);

        let msg = ledger2_service
            .build_msg_bank_send(&alice.account_id, &bob.account_id, 2, "token")
            .unwrap();

        let tx = pool2_service
            .build_tx(
                &alice.pub_key,
                vec![msg],
                "verimcosmos",
                9, // What is it?
                0,
                300000,
                0u64,
                "stake",
                10000,
                "memo",
            )
            .unwrap();

        let signed = keys_service.sign("alice", tx).await.unwrap();

        // Broadcast

        pool2_service
            .send_tx_commit(signed, "http://localhost:26657")
            .await
            .unwrap();

        assert!(true)
    }
    //
    // #[async_std::test]
    // async fn test_create_nym_flow() {
    //     let ledger2_service = Ledger2Service::new();
    //     let pool2_service = Pool2Service::new();
    //     let keys_service = KeysService::new();
    //
    //     let alice = keys_service
    //         .add_from_mnemonic("alice", "alice")
    //         .await
    //         .unwrap();
    //     let bob = keys_service.add_from_mnemonic("bob", "bob").await.unwrap();
    //
    //     println!("Alice's account id: {}", alice.account_id);
    //     println!("Bob's account id: {}", bob.account_id);
    //
    //     let msg = ledger2_service
    //         .build_msg_create_nym("alias", "verkey", "did", "role", alice.alias.as_str())
    //         .unwrap();
    //
    //     let tx = pool2_service
    //         .build_tx(
    //             &alice.pub_key,
    //             vec![msg],
    //             "verimcosmos",
    //             9, // What is it?
    //             0,
    //             300000,
    //             0u64,
    //             "stake",
    //             3909860,
    //             "memo",
    //         )
    //         .unwrap();
    //
    //     let signed = keys_service.sign("alice", tx).await.unwrap();
    //
    //     // Broadcast
    //
    //     pool2_service
    //         .send_tx_commit(signed, "http://localhost:26657")
    //         .await
    //         .unwrap();
    //
    //     assert!(true)
    // }
}
