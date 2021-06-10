use std::sync::Arc;
use crate::services::VerimLedgerService;
use indy_api_types::errors::IndyResult;
use cosmos_sdk::rpc::endpoint::abci_query::Response as QueryResponse;
use crate::services::{CosmosKeysService, CosmosLedgerService, TendermintPoolService};
use cosmos_sdk::tx::Msg;
use crate::domain::verim_ledger::cosmos_ext::{CosmosMsgExt, CosmosSignDocExt};

pub(crate) struct CosmosLedgerController {
    cosmos_ledger_service: Arc<CosmosLedgerService>,
    cosmos_keys_service: Arc<CosmosKeysService>,
    tendermint_pool_service: Arc<TendermintPoolService>
}

impl CosmosLedgerController {
    pub(crate) fn new(
        cosmos_ledger_service: Arc<CosmosLedgerService>,
        cosmos_keys_service: Arc<CosmosKeysService>,
        tendermint_pool_service: Arc<TendermintPoolService>

    ) -> Self {
        Self {
            cosmos_ledger_service,
            cosmos_keys_service,
            tendermint_pool_service
        }
    }

    pub(crate) async fn build_tx(
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
        trace!("build_tx > pool_alias {:?}, sender_alias {:?}, msg {:?}, account_number {:?}, sequence_number {:?}, max_gas {:?}, max_coin_amount {:?}, max_coin_denom {:?}, timeout_height {:?}, memo {:?}", pool_alias, sender_alias, msg, account_number, sequence_number, max_gas, max_coin_amount, max_coin_denom, timeout_height, memo);

        let pool = self.tendermint_pool_service.get_config(pool_alias).await?;
        let sender = self.cosmos_keys_service.get_info(sender_alias).await?;
        let msg = Msg::from_bytes(&msg)?;

        let sign_doc = self
            .cosmos_ledger_service
            .build_tx(
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

        trace!("build_tx <");

        Ok(sign_doc.to_bytes()?)
    }

    pub(crate) fn build_query_cosmos_auth_account(&self, address: &str) -> IndyResult<String> {
        trace!("build_query_cosmos_auth_account >");
        let query = self.cosmos_ledger_service.build_query_cosmos_auth_account(address)?;
        let json = serde_json::to_string(&query)?;
        trace!("build_query_cosmos_auth_account < {:?}", query);
        Ok(json)
    }

    pub(crate) fn parse_query_cosmos_auth_account_resp(&self, resp_json: &str) -> IndyResult<String> {
        trace!("parse_query_cosmos_auth_account_resp > resp {:?}", resp_json);
        let resp: QueryResponse = serde_json::from_str(resp_json)?;
        let result = self.cosmos_ledger_service.parse_query_cosmos_auth_account_resp(&resp)?;
        let json_result = serde_json::to_string(&result)?;
        trace!("parse_query_cosmos_auth_account_resp < {:?}", json_result);
        Ok(json_result)
    }
}