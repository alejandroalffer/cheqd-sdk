use std::str::FromStr;

use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::Msg;
use cosmos_sdk::tx::MsgType;
use indy_api_types::errors::IndyResult;
use log_derive::logfn;

use crate::domain::cheqd_ledger::base::query::PageRequest;
use crate::domain::cheqd_ledger::prost_ext::ProstMessageExt;
use crate::domain::cheqd_ledger::CheqdProto;
use crate::domain::cheqd_ledger::bank::{MsgSend, Coin, MsgSendResponse, QueryBalanceRequest, QueryBalanceResponse};

pub(crate) struct CheqdBankService {}

impl CheqdXferService {
    #[logfn(Info)]
    pub(crate) fn bank_build_msg_send(
        &self,
        from_address: &str,
        to_address: &str,
        amount: Vec<Coin>
    ) -> IndyResult<Msg> {
        let msg_send = MsgSend::new(
            from_address.to_string(),
            to_address.to_string(),
            amount
        );

        Ok(msg_send.to_proto().to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn bank_parse_msg_send_resp(
        &self,
        resp: &Response
    ) -> IndyResult<MsgSendResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn bank_build_query_balance(
        &self,
        address: String,
        amount: String
    ) -> IndyResult<abci_query::Request> {
        let query_data = QueryBalanceRequest::new(address, amount);
        let path = format!("/cheqdid.cheqdnode.bank.Query/Balance");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn bank_build_query_balance_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryBalanceResponse> {
        let result = QueryBalanceResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }
}
