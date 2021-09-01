use std::str::FromStr;

use cosmrs::rpc::endpoint::abci_query;
use cosmrs::rpc::endpoint::broadcast::tx_commit::Response;
use cosmrs::tx::Msg;
use cosmrs::tx::MsgType;
use indy_api_types::errors::IndyResult;
use log_derive::logfn;

use crate::domain::cheqd_ledger::prost_ext::ProstMessageExt;
use crate::domain::cheqd_ledger::CheqdProto;
use crate::domain::cheqd_ledger::bank::{MsgSend, Coin, MsgSendResponse, QueryBalanceRequest, QueryBalanceResponse};
use crate::services::CheqdLedgerService;

impl CheqdLedgerService {
    fn get_vector_coins_from_amount_and_denom(
        &self,
        amount: &str,
        denom: &str
    ) -> IndyResult<Vec<Coin>> {
        let coin = Coin::new(denom.to_string(), amount.to_string());
        let mut coins = Vec::new();
        coins.push(coin);

        Ok(coins)
    }

    #[logfn(Info)]
    pub(crate) fn bank_build_msg_send(
        &self,
        from_address: &str,
        to_address: &str,
        amount: &str,
        denom: &str
    ) -> IndyResult<Msg> {
        let coins: Vec<Coin> = self.get_vector_coins_from_amount_and_denom(amount, denom)?;
        let msg_send = MsgSend::new(
            from_address.to_string(),
            to_address.to_string(),
            coins
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
        denom: String
    ) -> IndyResult<abci_query::Request> {
        let query_data = QueryBalanceRequest::new(address, denom);
        let path = format!("/cosmos.bank.v1beta1.Query/Balance");
        let path = cosmrs::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn bank_parse_query_balance_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryBalanceResponse> {
        let result = QueryBalanceResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }
}
