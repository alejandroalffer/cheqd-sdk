//! Cosmos key management service

use crate::services::KeysService;
use async_std::sync::Arc;
use indy_api_types::errors::IndyResult;
use crate::domain::keys::KeyInfo;
use indy_api_types::IndyError;
use cosmos_sdk::tx::{Raw, SignDoc};

pub(crate) struct KeysController {
    keys_service: Arc<KeysService>,
}

impl KeysController {
    pub(crate) fn new(keys_service: Arc<KeysService>) -> Self {
        Self { keys_service }
    }


    pub(crate) async fn add_random(
        &self,
        alias: &str
    ) -> IndyResult<KeyInfo> {
        trace!(
            "add_random > alias {:?}",
            alias
        );

        let res = self.keys_service.add_random(&alias).await;
        trace!("add_random < {:?}", res);
        res
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
        alias: &str,
        mnemonic: &str
    ) -> IndyResult<KeyInfo> {
        trace!(
            "add_from_mnemonic > alias {:?}",
            alias
        );

        let res = self.keys_service.add_from_mnemonic(&alias, mnemonic).await;
        trace!("add_from_mnemonic < {:?}", res);
        res
    }

    pub(crate) async fn key_info(
        &self,
        alias: &str
    ) -> IndyResult<KeyInfo> {
        trace!(
            "key_info > alias {:?}",
            alias
        );

        let res = self.keys_service.key_info(&alias).await;
        trace!("key_info < {:?}", res);
        res
    }

}
