//! Cosmos pool management service

use async_std::sync::Arc;
use cosmrs::rpc;
use cosmrs::tx::Raw;
use indy_api_types::errors::{IndyResult, IndyErrorKind, IndyResultExt};

use crate::services::CheqdPoolService;

pub(crate) struct CheqdPoolController {
    cheqd_pool_service: Arc<CheqdPoolService>,
}

impl CheqdPoolController {
    pub(crate) fn new(
        cheqd_pool_service: Arc<CheqdPoolService>,
    ) -> Self {
        Self {
            cheqd_pool_service,
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
            .cheqd_pool_service
            .add(alias, rpc_address, chain_id)
            .await?;
        let json = serde_json::to_string(&config).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize PoolConfig object"
        )?;
        trace!("add < {:?}", json);
        Ok(json)
    }

    pub(crate) async fn get_config(&self, alias: &str) -> IndyResult<String> {
        trace!("get_config > alias {:?}", alias);
        let config = self.cheqd_pool_service.get_config(alias).await?;
        let json = serde_json::to_string(&config).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize PoolConfig object"
        )?;
        trace!("get_config < {:?}", json);
        Ok(json)
    }

    pub(crate) async fn get_all_config(&self) -> IndyResult<String> {
        trace!("get_config >");
        let config = self.cheqd_pool_service.get_all_config().await?;
        let json = serde_json::to_string(&config).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize list of PoolConfig objects"
        )?;
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
            .cheqd_pool_service
            .broadcast_tx_commit(pool_alias, tx_raw)
            .await?;
        let json = serde_json::to_string(&resp).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize Response object after broadcasting_tx_commit action"
        )?;

        trace!("broadcast_tx_commit < resp {:?}", json);

        Ok(json)
    }

    pub(crate) async fn abci_query(&self, pool_alias: &str, req_json: &str) -> IndyResult<String> {
        let req: rpc::endpoint::abci_query::Request = serde_json::from_str(req_json).to_indy(
            IndyErrorKind::InvalidStructure,
            "Cannot deserialize string of ABCI Response object"
        )?;
        let resp = self.cheqd_pool_service.abci_query(pool_alias, req).await?;
        let json_resp = serde_json::to_string(&resp).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize ABCI Response object"
        )?;
        Ok(json_resp)
    }

    pub(crate) async fn abci_info(&self, pool_alias: &str) -> IndyResult<String> {
        let resp = self.cheqd_pool_service.abci_info(pool_alias).await?;
        let json_resp = serde_json::to_string(&resp).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize Response after sending ABCI info request"
        )?;
        Ok(json_resp)
    }
}
