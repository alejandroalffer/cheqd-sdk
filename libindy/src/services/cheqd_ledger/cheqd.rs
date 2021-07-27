use std::str::FromStr;

use cosmos_sdk::rpc::endpoint::abci_query;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;
use cosmos_sdk::tx::Msg;
use cosmos_sdk::tx::MsgType;
use indy_api_types::errors::IndyResult;
use log_derive::logfn;

use crate::domain::cheqd_ledger::base::query::PageRequest;
use crate::domain::cheqd_ledger::prost_ext::ProstMessageExt;
use crate::domain::cheqd_ledger::cheqd::messages::MsgCreateNym;
use crate::domain::cheqd_ledger::cheqd::messages::MsgCreateNymResponse;
use crate::domain::cheqd_ledger::cheqd::messages::MsgDeleteNym;
use crate::domain::cheqd_ledger::cheqd::messages::MsgDeleteNymResponse;
use crate::domain::cheqd_ledger::cheqd::messages::MsgUpdateNym;
use crate::domain::cheqd_ledger::cheqd::messages::MsgUpdateNymResponse;
use crate::domain::cheqd_ledger::cheqd::queries::QueryAllNymRequest;
use crate::domain::cheqd_ledger::cheqd::queries::QueryAllNymResponse;
use crate::domain::cheqd_ledger::cheqd::queries::QueryGetNymRequest;
use crate::domain::cheqd_ledger::cheqd::queries::QueryGetNymResponse;
use crate::domain::cheqd_ledger::CheqdProto;
use crate::services::CheqdLedgerService;

impl CheqdLedgerService {
    #[logfn(Info)]
    pub(crate) fn cheqd_build_msg_create_nym(
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

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_msg_create_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgCreateNymResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_build_msg_update_nym(
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

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_msg_update_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgUpdateNymResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Msg> {
        let msg_send = MsgDeleteNym {
            creator: creator.to_string(),
            id,
        };

        Ok(msg_send.to_proto().to_msg()?)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_msg_delete_nym_resp(
        &self,
        resp: &Response,
    ) -> IndyResult<MsgDeleteNymResponse> {
        self.parse_msg_resp(resp)
    }

    #[logfn(Info)]
    pub(crate) fn build_query_get_nym(&self, id: u64) -> IndyResult<abci_query::Request> {
        let query_data = QueryGetNymRequest::new(id);
        let path = format!("/cheqdid.cheqdnode.cheqd.Query/Nym");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_query_get_nym_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryGetNymResponse> {
        let result = QueryGetNymResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_build_query_all_nym(
        &self,
        pagination: Option<PageRequest>,
    ) -> IndyResult<abci_query::Request> {
        let query_data = QueryAllNymRequest::new(pagination);
        let path = format!("/cheqdid.cheqdnode.cheqd.Query/NymAll");
        let path = cosmos_sdk::tendermint::abci::Path::from_str(&path)?;
        let req =
            abci_query::Request::new(Some(path), query_data.to_proto().to_bytes()?, None, true);
        Ok(req)
    }

    #[logfn(Info)]
    pub(crate) fn cheqd_parse_query_all_nym_resp(
        &self,
        resp: &abci_query::Response,
    ) -> IndyResult<QueryAllNymResponse> {
        let result = QueryAllNymResponse::from_proto_bytes(&resp.response.value)?;
        return Ok(result);
    }
}
