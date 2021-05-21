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

    pub async fn add(&self, alias: &str, rpc_address: &str, chain_id: &str) -> IndyResult<()> {
        trace!(
            "add > alias {:?} rpc_address {:?} chain_id {:?}",
            alias,
            rpc_address,
            chain_id
        );
        self.cosmos_pool_service
            .add(alias, rpc_address, chain_id)
            .await?;
        trace!("add <");
        Ok(())
    }

    pub async fn pool_config(&self, alias: &str) -> IndyResult<CosmosPoolConfig> {
        trace!("pool_config > alias {:?}", alias);
        let config = self.cosmos_pool_service.pool_config(alias).await?;
        trace!("pool_config <");
        Ok(config)
    }

    // Returns tx bytes.
    pub async fn build_tx(
        &self,
        pool_alias: &str,
        sender_alias: &str,
        msg: &[u8],
        account_number: u64,
        sequence_number: u64,
        max_gas: u64,
        max_coin_amount: u64,
        max_coin_denom: &str,
        timeout_height: u16,
        memo: &str,
    ) -> IndyResult<Vec<u8>> {
        trace!("build_tx > pool_alias {:?}, sender_alias {:?}, msg {:?}, account_number {:?}, sequence_number {:?}, max_gas {:?}, max_coin_amount {:?}, max_coin_denom {:?}, timeout_height {:?}, memo {:?}", pool_alias, sender_alias, msg, account_number, sequence_number, max_gas, max_coin_amount, max_coin_denom, timeout_height, memo);

        let pool = self.cosmos_pool_service.pool_config(pool_alias).await?;
        let sender = self.cosmos_keys_service.key_info(sender_alias).await?;
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

    // Send and wait for commit
    pub async fn broadcast_tx_commit(
        &self,
        pool_alias: &str,
        signed_tx: &[u8],
    ) -> IndyResult<rpc::endpoint::broadcast::tx_commit::Response> {
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

        trace!("broadcast_tx_commit < resp_bytes {:?}", resp);

        Ok(resp)
    }
}
