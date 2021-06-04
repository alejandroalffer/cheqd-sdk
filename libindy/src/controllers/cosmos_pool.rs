//! Cosmos pool management service

use async_std::sync::Arc;
use cosmos_sdk::rpc;
use cosmos_sdk::tx::{Msg, Raw};
use indy_api_types::errors::IndyResult;

use crate::domain::cosmos_pool::CosmosPoolConfig;
use crate::domain::verim_ledger::cosmos_ext::{CosmosMsgExt, CosmosSignDocExt};
use crate::services::{CosmosKeysService, CosmosPoolService};

pub(crate) struct CosmosPoolController {
    cosmos_pool_service: Arc<CosmosPoolService>,
    cosmos_keys_service: Arc<CosmosKeysService>,
}

impl CosmosPoolController {
    pub(crate) fn new(
        cosmos_pool_service: Arc<CosmosPoolService>,
        cosmos_keys_service: Arc<CosmosKeysService>,
    ) -> Self {
        Self {
            cosmos_pool_service,
            cosmos_keys_service,
        }
    }

    pub(crate) async fn add(
        &self,
        alias: &str,
        rpc_address: &str,
        chain_id: &str,
    ) -> IndyResult<String> {
        trace!(
            "add > alias {:?} rpc_address {:?} chain_id {:?}",
            alias,
            rpc_address,
            chain_id
        );
        let config = self
            .cosmos_pool_service
            .add(alias, rpc_address, chain_id)
            .await?;
        let json = serde_json::to_string(&config)?;
        trace!("add < {:?}", json);
        Ok(json)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<String> {
        trace!("get_config > alias {:?}", alias);
        let config = self.cosmos_pool_service.get_config(alias).await?;
        let json = serde_json::to_string(&config)?;
        trace!("get_config < {:?}", json);
        Ok(json)
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

        let pool = self.cosmos_pool_service.get_config(pool_alias).await?;
        let sender = self.cosmos_keys_service.get_info(sender_alias).await?;
        let msg = Msg::from_bytes(&msg)?;

        let sign_doc = self
            .cosmos_pool_service
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

    pub(crate) async fn broadcast_tx_commit(
        &self,
        pool_alias: &str,
        signed_tx: &[u8],
    ) -> IndyResult<String> {
        trace!(
            "broadcast_tx_commit > pool_alias {:?}, signed_tx {:?}",
            pool_alias,
            signed_tx
        );

        let tx_raw = Raw::from_bytes(signed_tx)?;
        let resp = self
            .cosmos_pool_service
            .broadcast_tx_commit(pool_alias, tx_raw)
            .await?;
        let json = serde_json::to_string(&resp)?;

        trace!("broadcast_tx_commit < resp {:?}", json);

        Ok(json)
    }

    pub(crate) async fn abci_query(&self, pool_alias: &str, req_json: &str) -> IndyResult<String> {
        let req: rpc::endpoint::abci_query::Request = serde_json::from_str(req_json)?;
        let resp = self.cosmos_pool_service.abci_query(pool_alias, req).await?;
        let json_resp = serde_json::to_string(&resp)?;
        Ok(json_resp)
    }
}
