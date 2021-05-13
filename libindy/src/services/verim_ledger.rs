//! Ledger service for Verim back-end

use std::convert::TryInto;
use std::str::FromStr;

use crate::domain::crypto::did::DidValue;
use crate::domain::verim_ledger::cosmos_ext::CosmosMsgExt;
use crate::domain::verim_ledger::cosmos_ext::ProstMessageExt;
use crate::domain::verim_ledger::verimcosmos::messages::MsgDeleteNym;
use crate::domain::verim_ledger::verimcosmos::messages::MsgUpdateNym;
use crate::domain::verim_ledger::verimcosmos::messages::{MsgCreateNym, MsgCreateNymResponse};
use crate::domain::verim_ledger::VerimMessage;
use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::{Fee, Msg, MsgProto, MsgType, SignDoc, SignerInfo};
use cosmos_sdk::Coin;
use cosmos_sdk::{rpc, tx};
use hex::FromHex;
use indy_api_types::errors::prelude::*;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use indy_utils::crypto::hash::hash as openssl_hash;
use log_derive::logfn;
use prost::Message;
use prost_types::Any;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};

pub(crate) struct VerimLedgerService {}

impl VerimLedgerService {
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

        Ok(msg_send.to_msg()?)
    }

    pub(crate) fn parse_msg_crate_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgCreateNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();

        // let any = Any::from_bytes(data);
        let res2 = crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::MsgCreateNymResponse::from_bytes(data);
        // let msg = Msg::from_bytes(&data)?; // Any
        // let parsed = MsgCreateNymResponse::from_msg(&msg)?;
        //
        Err(IndyError::from_msg(IndyErrorKind::InvalidState, ""))
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_create_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
    ) -> IndyResult<Msg> {
        let msg_send = MsgCreateNym {
            creator: creator.to_string(),
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: role.to_string(),
        };

        Ok(msg_send.to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_update_nym(
        &self,
        creator: &str,
        id: u64,
        verkey: &str,
        alias: &str,
        did: &str,
        role: &str,
    ) -> IndyResult<Msg> {
        let msg_send = MsgUpdateNym {
            creator: creator.to_string(),
            id,
            alias: alias.to_string(),
            verkey: verkey.to_string(),
            did: did.to_string(),
            role: role.to_string(),
        };

        Ok(msg_send.to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Msg> {
        let msg_send = MsgDeleteNym {
            creator: creator.to_string(),
            id,
        };

        Ok(msg_send.to_msg()?)
    }

    // TODO: Queries
    pub(crate) fn build_query_cosmos_account(
        &self,
        address: &str,
    ) -> IndyResult<abci_query::Request> {
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

    // TODO: Queries
    pub(crate) fn build_query_verimcosmos_list_nym(&self) -> IndyResult<abci_query::Request> {
        let path = format!("custom/verimcosmos/list-nym");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req = abci_query::Request::new(Some(path), Vec::new(), None, false);
        Ok(req)
    }

    // TODO: Queries
    pub(crate) fn build_query_verimcosmos_get_nym(
        &self,
        id: &str,
    ) -> IndyResult<abci_query::Request> {
        let path = format!("custom/verimcosmos/get-nym/{}", id);
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req = abci_query::Request::new(Some(path), Vec::new(), None, false);
        Ok(req)
    }
}

#[cfg(test)]
mod test {
    use cosmos_sdk::crypto::secp256k1::SigningKey;
    use prost::Message;
    use rust_base58::ToBase58;

    use crate::domain::cosmos_pool::CosmosPoolConfig;
    use crate::domain::crypto::did::DidValue;
    use crate::services::{CosmosKeysService, CosmosPoolService, VerimLedgerService};

    #[async_std::test]
    async fn test_query_list_nym() {
        let verim_ledger_service = VerimLedgerService::new();
        let cosmos_pool_service = CosmosPoolService::new();
        let cosmos_keys_service = CosmosKeysService::new();

        let req = verim_ledger_service
            .build_query_verimcosmos_list_nym()
            .unwrap();

        let result = cosmos_pool_service
            .abci_query("http://localhost:26657", req)
            .await
            .unwrap();

        let inner = result.response.value;

        let decoded =
            crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos::QueryAllNymResponse::decode(inner.as_slice())
                .unwrap();

        // QueryAllNymResponse

        println!("{:?}", decoded);
    }

    #[async_std::test]
    async fn test_query_account() {
        let verim_ledger_service = VerimLedgerService::new();
        let cosmos_pool_service = CosmosPoolService::new();
        let cosmos_keys_service = CosmosKeysService::new();

        let req = verim_ledger_service
            .build_query_cosmos_account("cosmos17gt4any4r9jgg06r47f83vfxrycdk67utjs36m")
            .unwrap();

        let result = cosmos_pool_service
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
