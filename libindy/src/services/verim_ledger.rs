//! Ledger service for Verim back-end

use std::convert::TryInto;
use std::str::FromStr;

use crate::domain::crypto::did::DidValue;
use crate::domain::verim_ledger::prost_ext::ProstMessageExt;
use crate::domain::verim_ledger::verimcosmos::messages::MsgUpdateNym;
use crate::domain::verim_ledger::verimcosmos::messages::{MsgCreateNym, MsgCreateNymResponse};
use crate::domain::verim_ledger::verimcosmos::messages::{
    MsgDeleteNym, MsgDeleteNymResponse, MsgUpdateNymResponse,
};
use crate::domain::verim_ledger::VerimProto;
use cosmos_sdk::bank::MsgSend;
use cosmos_sdk::proto::cosmos::base::abci::v1beta1::{MsgData, TxMsgData};
use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::{Fee, Msg, MsgProto, MsgType, SignDoc, SignerInfo};
use cosmos_sdk::Coin;
use cosmos_sdk::{rpc, tx};
use hex::FromHex;
use indy_api_types::errors::prelude::*;
use indy_api_types::errors::{IndyErrorKind, IndyResult};
use indy_api_types::IndyError;
use log_derive::logfn;
use prost::Message;
use prost_types::Any;
use serde::de::DeserializeOwned;
use serde_json::{self, Value};

use crate::domain::verim_ledger::cosmos::auth::{QueryAccountRequest, QueryAccountResponse};
use crate::domain::verim_ledger::cosmos::base::query::PageRequest;
use crate::domain::verim_ledger::proto::verimid::verimcosmos::verimcosmos;
use crate::domain::verim_ledger::verimcosmos::queries::{
    QueryAllNymRequest, QueryAllNymResponse, QueryGetNymRequest, QueryGetNymResponse,
};

pub(crate) struct VerimLedgerService {}

impl VerimLedgerService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn parse_msg_create_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgCreateNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let result = MsgCreateNymResponse::from_proto_bytes(&tx_msg.data[0].data)?;

        return Ok(result);
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
        let msg_send = MsgCreateNym::new(
            creator.to_string(),
            alias.to_string(),
            verkey.to_string(),
            did.to_string(),
            role.to_string(),
        );

        Ok(msg_send.to_proto().to_msg()?)
    }

    pub(crate) fn parse_msg_update_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgUpdateNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let result = MsgUpdateNymResponse::from_proto_bytes(&tx_msg.data[0].data)?;

        return Ok(result);
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_update_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
        id: u64,
    ) -> IndyResult<Msg> {
        let msg_send = MsgUpdateNym::new(
            creator.to_string(),
            id,
            alias.to_string(),
            verkey.to_string(),
            did.to_string(),
            role.to_string(),
        );

        Ok(msg_send.to_proto().to_msg()?)
    }

    pub(crate) fn parse_msg_delete_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgDeleteNymResponse> {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let result = MsgDeleteNymResponse::from_proto_bytes(&tx_msg.data[0].data)?;

        return Ok(result);
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Msg> {
        let msg_send = MsgDeleteNym {
            creator: creator.to_string(),
            id,
        };

        Ok(msg_send.to_proto().to_msg()?)
    }

    pub(crate) fn build_query_verimcosmos_list_nym(&self) -> IndyResult<abci_query::Request> {
        let path = format!("custom/verimcosmos/list-nym");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req = abci_query::Request::new(Some(path), Vec::new(), None, false);
        Ok(req)
    }

    pub(crate) fn build_query_get_nym(&self, id: u64) -> IndyResult<abci_query::Request> {
        let query_data = QueryGetNymRequest::new(id);
        let path = format!("/verimid.verimcosmos.verimcosmos.Query/Nym");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    pub(crate) fn parse_query_get_nym_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryGetNymResponse> {
        let result = QueryGetNymResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }

    pub(crate) fn build_query_all_nym(
        &self,
        pagination: Option<PageRequest>,
    ) -> IndyResult<abci_query::Request> {
        let query_data = QueryAllNymRequest::new(pagination);
        let path = format!("/verimid.verimcosmos.verimcosmos.Query/NymAll");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    pub(crate) fn parse_query_all_nym_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryAllNymResponse> {
        let result = QueryAllNymResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }
}
