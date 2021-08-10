use crate::controllers::CheqdLedgerController;
use indy_api_types::errors::IndyResult;
use crate::domain::cheqd_ledger::cosmos_ext::CosmosMsgExt;
use cosmos_sdk::rpc::endpoint::abci_query::Response as QueryResponse;
use cosmos_sdk::rpc::endpoint::broadcast::tx_commit::Response;

impl CheqdLedgerController {
    pub(crate) fn bank_build_msg_send(
        &self,
        from_address: &str,
        to_address: &str,
        amount: &str,
        denom: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!(
            "bank_build_msg_send > from_address {:?} to_address {:?} amount {:?}, denom {:?}",
            from_address,
            to_address,
            amount,
            denom
        );
        let msg = self
            .cheqd_ledger_service
            .bank_build_msg_send(from_address, to_address, amount, denom)?;
        trace!("bank_build_msg_send < {:?}", msg);

        Ok(msg.to_bytes()?)
    }

    pub(crate) fn bank_parse_msg_send_resp(&self, resp: &str) -> IndyResult<String> {
        trace!("bank_parse_msg_send_resp > resp {:?}", resp);
        let resp: Response = serde_json::from_str(&resp)?;
        let res = self.cheqd_ledger_service.bank_parse_msg_send_resp(&resp)?;
        let res = serde_json::to_string(&res)?;
        trace!("bank_parse_msg_send_resp < {:?}", res);
        Ok(res)
    }

    pub(crate) fn bank_build_query_balance(&self, address: String, denom: String) -> IndyResult<String> {
        trace!("bank_build_query_balance > address {:?} denom {:?}", address, denom);
        let query = self.cheqd_ledger_service.bank_build_query_balance(address, denom)?;
        let json = serde_json::to_string(&query)?;
        trace!("bank_build_query_balance < {:?}", query);
        Ok(json)
    }

    pub(crate) fn bank_parse_query_balance_resp(&self, resp_json: &str) -> IndyResult<String> {
        trace!("bank_parse_query_balance_resp > resp {:?}", resp_json);
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self.cheqd_ledger_service.bank_parse_query_balance_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("bank_parse_query_balance_resp < {:?}", json_result);
        Ok(json_result)
    }
}
