//! Cosmos pool management service

use async_std::sync::Arc;
use cosmos_sdk::rpc;
use cosmos_sdk::tx::{Msg, Raw};
use indy_api_types::errors::IndyResult;

use crate::domain::verim_pool::VerimPoolConfig;
use crate::domain::verim_ledger::cosmos_ext::{CosmosMsgExt, CosmosSignDocExt};
use crate::services::{VerimKeysService, VerimPoolService};

pub(crate) struct VerimPoolController {
    verim_pool_service: Arc<VerimPoolService>,
}

impl VerimPoolController {
    pub(crate) fn new(
        verim_pool_service: Arc<VerimPoolService>,
    ) -> Self {
        Self {
            verim_pool_service,
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
            .verim_pool_service
            .add(alias, rpc_address, chain_id)
            .await?;
        let json = serde_json::to_string(&config)?;
        trace!("add < {:?}", json);
        Ok(json)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<String> {
        trace!("get_config > alias {:?}", alias);
        let config = self.verim_pool_service.get_config(alias).await?;
        let json = serde_json::to_string(&config)?;
        trace!("get_config < {:?}", json);
        Ok(json)
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
            .verim_pool_service
            .broadcast_tx_commit(pool_alias, tx_raw)
            .await?;
        let json = serde_json::to_string(&resp)?;

        trace!("broadcast_tx_commit < resp {:?}", json);

        Ok(json)
    }

    pub(crate) async fn abci_query(&self, pool_alias: &str, req_json: &str) -> IndyResult<String> {
        let req: rpc::endpoint::abci_query::Request = serde_json::from_str(req_json)?;
        let resp = self.verim_pool_service.abci_query(pool_alias, req).await?;
        let json_resp = serde_json::to_string(&resp)?;
        Ok(json_resp)
    }

    pub(crate) async fn abci_info(&self, pool_alias: &str) -> IndyResult<String> {
        let resp = self.verim_pool_service.abci_info(pool_alias).await?;
        let json_resp = serde_json::to_string(&resp)?;
        Ok(json_resp)
    }
}
