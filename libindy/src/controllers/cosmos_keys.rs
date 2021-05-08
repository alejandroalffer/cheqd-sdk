//! Cosmos key management service

use crate::domain::cosmos_keys::KeyInfo;
use crate::services::CosmosKeysService;
use async_std::sync::Arc;
use cosmos_sdk::tx::{Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use indy_api_types::IndyError;

pub(crate) struct CosmosKeysController {
    keys_service: Arc<CosmosKeysService>,
}

impl CosmosKeysController {
    pub(crate) fn new(keys_service: Arc<CosmosKeysService>) -> Self {
        Self { keys_service }
    }

    pub(crate) async fn add_random(&self, alias: &str) -> IndyResult<KeyInfo> {
        trace!("add_random > alias {:?}", alias);
        let res = self.keys_service.add_random(&alias).await;
        trace!("add_random < {:?}", res);
        res
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
        alias: &str,
        mnemonic: &str,
    ) -> IndyResult<KeyInfo> {
        trace!("add_from_mnemonic > alias {:?}", alias);
        let res = self.keys_service.add_from_mnemonic(&alias, mnemonic).await;
        trace!("add_from_mnemonic < {:?}", res);
        res
    }

    pub(crate) async fn key_info(&self, alias: &str) -> IndyResult<KeyInfo> {
        trace!("key_info > alias {:?}", alias);
        let res = self.keys_service.key_info(&alias).await;
        trace!("key_info < {:?}", res);
        res
    }
}
