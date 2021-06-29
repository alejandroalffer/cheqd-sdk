use crate::controllers::VerimLedgerController;
use indy_api_types::errors::IndyResult;
use crate::domain::verim_ledger::cosmos_ext::CosmosMsgExt;
use cosmos_sdk::rpc::endpoint::abci_query::Response as QueryResponse;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;

impl VerimLedgerController {
    pub(crate) fn verim_build_msg_create_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "verim_build_msg_create_nym > did {:?} creator {:?} verkey {:?} alias {:?} role {:?}",
            did,
            creator,
            verkey,
            alias,
            role
        );
        let msg = self
            .verim_ledger_service
            .verim_build_msg_create_nym(did, creator, verkey, alias, role)?;
        trace!("verim_build_msg_create_nym < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn verim_parse_msg_create_nym_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("verim_parse_msg_create_nym_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(&resp)?;
        let res = self.verim_ledger_service.verim_parse_msg_create_nym_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("verim_parse_msg_create_nym_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn verim_build_msg_update_nym(
        &self,
        did: &str,
        creator: &str,
        verkey: &str,
        alias: &str,
        role: &str,
        id: u64,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "verim_build_msg_update_nym > creator {:?} verkey {:?} alias {:?} did {:?} role {:?} id {:?}",
            did,
            creator,
            verkey,
            alias,
            role,
            id
        );
        let msg = self
            .verim_ledger_service
            .verim_build_msg_update_nym(did, creator, verkey, alias, role, id)?;
        trace!("verim_build_msg_update_nym < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn verim_parse_msg_update_nym_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("verim_parse_msg_update_nym_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(resp)?;
        let res = self.verim_ledger_service.verim_parse_msg_update_nym_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("verim_parse_msg_update_nym_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn verim_build_msg_delete_nym(&self, creator: &str, id: u64) -> IndyResult<Vec<u8>> {
        trace!("verim_build_msg_delete_nym > creator {:?} id {:?}", creator, id,);
        let msg = self
            .verim_ledger_service
            .build_msg_delete_nym(creator, id)?;
        trace!("verim_build_msg_delete_nym < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn verim_parse_msg_delete_nym_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("verim_parse_msg_delete_nym_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(resp)?;
        let res = self.verim_ledger_service.verim_parse_msg_delete_nym_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("verim_parse_msg_delete_nym_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn verim_build_query_get_nym(&self, id: u64) -> IndyResult<String> {
        trace!("verim_build_query_get_nym > id {:?}", id);
        let query = self.verim_ledger_service.build_query_get_nym(id)?;
        let json = serde_json::to_string(&query)?;
        trace!("verim_build_query_get_nym < {:?}", query);
        Ok(json)
    }

    pub(crate) fn verim_parse_query_get_nym_resp(&self, resp_json: &str) -> IndyResult<String> {
        trace!("verim_parse_query_get_nym_resp > resp {:?}", resp_json);
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self.verim_ledger_service.verim_parse_query_get_nym_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("verim_parse_query_get_nym_resp < {:?}", json_result);
        Ok(json_result)
    }

    pub(crate) fn verim_build_query_all_nym(&self) -> IndyResult<String> {
        trace!("verim_build_query_all_nym >");
        let query = self.verim_ledger_service.verim_build_query_all_nym(None)?;
        let json = serde_json::to_string(&query)?;
        trace!("verim_build_query_all_nym < {:?}", query);
        Ok(json)
    }

    pub(crate) fn verim_parse_query_all_nym_resp(&self, resp_json: &str) -> IndyResult<String> {
        trace!("verim_parse_query_all_nym_resp > resp {:?}", resp_json);
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self.verim_ledger_service.verim_parse_query_all_nym_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("verim_parse_query_all_nym_resp < {:?}", json_result);
        Ok(json_result)
    }
}