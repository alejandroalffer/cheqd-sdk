use crate::controllers::VerimLedgerController;
use indy_api_types::errors::IndyResult;
use crate::domain::verim_ledger::cosmos_ext::{CosmosSignDocExt, CosmosMsgExt};
use cosmos_sdk::tx::Msg;
use cosmos_sdk::rpc::endpoint::abci_query::Response as QueryResponse;

impl VerimLedgerController {
    pub(crate) async fn auth_build_tx(
        &self,
        pool_alias: &str,
        sender_alias: &str,
        msg: &[u8],
        account_number: u64,
        sequence_number: u64,
        max_gas: u64,
        max_coin_amount: u64,
        max_coin_denom: &str,
        timeout_height: u64,
        memo: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!("auth_build_tx > pool_alias {:?}, sender_alias {:?}, msg {:?}, account_number {:?}, sequence_number {:?}, max_gas {:?}, max_coin_amount {:?}, max_coin_denom {:?}, timeout_height {:?}, memo {:?}", pool_alias, sender_alias, msg, account_number, sequence_number, max_gas, max_coin_amount, max_coin_denom, timeout_height, memo);

        let pool = self.verim_pool_service.get_config(pool_alias).await?;
        let sender = self.verim_keys_service.get_info(sender_alias).await?;
        let msg = Msg::from_bytes(&msg)?;

        let sign_doc = self
            .verim_ledger_service
            .auth_build_tx(
                &pool.chain_id,
                &sender.pub_key,
                msg,
                account_number,
                sequence_number,
                max_gas,
                max_coin_amount,
                max_coin_denom,
                timeout_height,
                memo,
            )
            .await?;

        trace!("auth_build_tx <");

        Ok(sign_doc.to_bytes()?)
    }

    pub(crate) fn auth_build_query_account(&self, address: &str) -> IndyResult<String> {
        trace!("auth_build_query_account >");
        let query = self
            .verim_ledger_service
            .auth_build_query_account(address)?;
        let json = serde_json::to_string(&query)?;
        trace!("auth_build_query_account < {:?}", query);
        Ok(json)
    }

    pub(crate) fn auth_parse_query_account_resp(
        &self,
        resp_json: &str,
    ) -> IndyResult<String> {
        trace!(
            "auth_parse_query_account_resp > resp {:?}",
            resp_json
        );
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self
            .verim_ledger_service
            .auth_parse_query_account_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("auth_parse_query_account_resp < {:?}", json_result);
        Ok(json_result)
    }
}