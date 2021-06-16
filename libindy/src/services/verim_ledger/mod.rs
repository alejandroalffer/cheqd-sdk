//! Ledger service for Verim back-end

use std::convert::TryInto;
use std::str::FromStr;

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

use crate::domain::crypto::did::DidValue;
use crate::domain::verim_ledger::cosmos::auth::{QueryAccountRequest, QueryAccountResponse};
use crate::domain::verim_ledger::cosmos::base::query::PageRequest;
use crate::domain::verim_ledger::prost_ext::ProstMessageExt;
use crate::domain::verim_ledger::verim::messages::MsgUpdateNym;
use crate::domain::verim_ledger::verim::messages::{MsgCreateNym, MsgCreateNymResponse};
use crate::domain::verim_ledger::verim::messages::{
    MsgDeleteNym, MsgDeleteNymResponse, MsgUpdateNymResponse,
};
use crate::domain::verim_ledger::verim::queries::{
    QueryAllNymRequest, QueryAllNymResponse, QueryGetNymRequest, QueryGetNymResponse,
};
use crate::domain::verim_ledger::VerimProto;

mod auth;
mod verim;

pub(crate) struct VerimLedgerService {}

impl VerimLedgerService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    #[logfn(Info)]
    fn parse_msg_resp<R>(&self, resp: &Response) -> IndyResult<R>
    where
        R: VerimProto,
    {
        let data = resp.deliver_tx.data.as_ref().ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Expected response data but got None",
        ))?;
        let data = data.value();
        let tx_msg = TxMsgData::from_bytes(&data)?;
        let result = R::from_proto_bytes(&tx_msg.data[0].data)?;

        return Ok(result);
    }
}
