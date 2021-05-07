//! Ledger service for Cosmos back-end

use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::rpc;
use cosmos_sdk::rpc::endpoint::abci_query;
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
use prost::Message;
use std::str::FromStr;

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
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str
    ) -> IndyResult<Msg> {
        let msg_send = verimid::verimcosmos::verimcosmos::MsgCreateNym {
            creator: creator.to_string(),
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: "role".to_string(),
        };

        Ok(msg_send.to_msg()?)
    }

    pub fn build_query_cosmos_account(&self, address: &str) -> IndyResult<abci_query::Request> {
        let path = "/cosmos.auth.v1beta1.Query/Account";
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;

        let data = cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountRequest {
            address: address.to_string(),
        };

        let mut data_bytes = Vec::new();
        data.encode(&mut data_bytes).unwrap(); // TODO

        let req = abci_query::Request::new(Some(path), data_bytes, None, false);
        Ok(req)
    }

    pub fn build_query_verimcosmos_list_nym(&self) -> IndyResult<abci_query::Request> {
        let path = format!("custom/verimcosmos/list-nym");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req = abci_query::Request::new(Some(path), Vec::new(), None, false);
        Ok(req)
    }

    pub fn build_query_verimcosmos_get_nym(&self, id: &str) -> IndyResult<abci_query::Request> {
        let path = format!("custom/verimcosmos/get-nym/{}", id);
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req = abci_query::Request::new(Some(path), Vec::new(), None, false);
        Ok(req)
    }
}

// pub trait

#[cfg(test)]
mod test {
    use crate::services::{KeysService, Ledger2Service, Pool2Service};
    use cosmos_sdk::crypto::secp256k1::SigningKey;
    use prost::Message;
    use rust_base58::ToBase58;
    use crate::domain::crypto::did::DidValue;

    #[async_std::test]
    async fn test_msg_bank_send() {
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
    async fn test_msg_create_nym() {
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
            .build_msg_create_nym("did", &*alice.account_id, "verkey", "bob")
            .unwrap();

        let tx = pool2_service
            .build_tx(
                &alice.pub_key,
                vec![msg],
                "verim-cosmos-chain",
                11,
                0,
                300000,
                300000u64,
                "token",
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

    #[async_std::test]
    async fn test_query_list_nym() {
        let ledger2_service = Ledger2Service::new();
        let pool2_service = Pool2Service::new();
        let keys_service = KeysService::new();

        let req = ledger2_service.build_query_verimcosmos_list_nym().unwrap();

        let result = pool2_service
            .abci_query("http://localhost:26657", req)
            .await
            .unwrap();

        let inner = result.response.value;

        let decoded =
            super::verimid::verimcosmos::verimcosmos::QueryAllNymResponse::decode(inner.as_slice())
                .unwrap();

        // QueryAllNymResponse

        println!("{:?}", decoded);
    }

    #[async_std::test]
    async fn test_query_account() {
        let ledger2_service = Ledger2Service::new();
        let pool2_service = Pool2Service::new();
        let keys_service = KeysService::new();

        let req = ledger2_service
            .build_query_cosmos_account("cosmos17gt4any4r9jgg06r47f83vfxrycdk67utjs36m")
            .unwrap();

        let result = pool2_service
            .abci_query("http://localhost:26657", req)
            .await
            .unwrap();

        let inner = result.response.value;

        let decoded = cosmos_sdk::proto::cosmos::auth::v1beta1::QueryAccountResponse::decode(
            inner.as_slice(),
        )
        .unwrap();

        let decoded = cosmos_sdk::proto::cosmos::auth::v1beta1::BaseAccount::decode(
            decoded.account.unwrap().value.as_slice(),
        )
        .unwrap();

        // QueryAllNymResponse

        println!("{:?}", decoded);
    }
}
