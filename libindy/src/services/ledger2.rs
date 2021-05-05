//! Ledger service for Cosmos back-end

use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::rpc;
use cosmos_sdk::tx::{Msg, MsgProto, MsgType};
use cosmos_sdk::Coin;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use hex::FromHex;
use indy_api_types::errors::prelude::*;
use indy_utils::crypto::hash::hash as openssl_hash;
use log_derive::logfn;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};
use ursa::cl::RevocationRegistryDelta as CryproRevocationRegistryDelta;
use crate::domain::crypto::did::DidValue;

pub mod verimid {
    pub mod verimcosmos {
        pub mod verimcosmos {
            include!(concat!(
                env!("OUT_DIR"),
                "/prost/verimid.verimcosmos.verimcosmos.rs"
            ));
        }
    }
}

impl MsgProto for verimid::verimcosmos::verimcosmos::MsgCreateNym {
    const TYPE_URL: &'static str = "/verimid.verimcosmos.verimcosmos.MsgCreateNym";
}

pub mod cosmos {
    pub mod base {
        pub mod query {
            pub mod v1beta1 {
                include!(concat!(
                    env!("OUT_DIR"),
                    "/prost/cosmos.base.query.v1beta1.rs"
                ));
            }
        }
    }
}

pub(crate) struct Ledger2Service {}

impl Ledger2Service {
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_bank_send(
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

        // TODO: Change result to bytes vec
        Ok(msg_send.to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_create_nym(
        &self,
        did: &DidValue,
        creator: &str,
        verkey: &str,
        alias: &str
    ) -> IndyResult<Msg> {
        let msg_send = verimid::verimcosmos::verimcosmos::MsgCreateNym {
            creator: creator.to_string(),
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: None,
        };

        Ok(msg_send.to_msg()?)
    }

    // pub fn build_query_account(&self, account_id: &str) -> IndyResult<(String, Vec<u8>)> {
    //     let path = "".to_owned();
    //
    //     let query = cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountRequest {
    //         address: account_id.to_string(),
    //     };
    //
    //
    //
    //     prost::Message::encode()
    //
    //     fn to_bytes(&self) -> Result<Vec<u8>> {
    //         let mut bytes = Vec::new();
    //         prost::Message::encode(self, &mut bytes)?;
    //         Ok(bytes)
    //     }
    //
    //         (path, query)
    // }
}

// pub trait

#[cfg(test)]
mod test {
    use crate::services::{KeysService, Ledger2Service, Pool2Service};
    use cosmos_sdk::crypto::secp256k1::SigningKey;
    use rust_base58::ToBase58;

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
            .broadcast_tx_commit(signed, "http://localhost:26657")
            .await
            .unwrap();

        assert!(true)
    }

    #[async_std::test]
    async fn test_create_nym_flow() {
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
            .build_msg_create_nym("alias", "verkey", "did", "role", &*alice.account_id)
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
                39090,
                "memo",
            )
            .unwrap();

        let signed = keys_service.sign("alice", tx).await.unwrap();

        // Broadcast

        pool2_service
            .broadcast_tx_commit(signed, "http://localhost:26657")
            .await
            .unwrap();

        assert!(true)
    }
}
