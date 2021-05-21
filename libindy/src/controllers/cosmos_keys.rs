//! Cosmos key management service

use crate::domain::cosmos_keys::KeyInfo;
use crate::domain::verim_ledger::cosmos_ext::CosmosSignDocExt;
use crate::services::CosmosKeysService;
use async_std::sync::Arc;
use cosmos_sdk::tx::SignDoc;
use indy_api_types::errors::IndyResult;

pub(crate) struct CosmosKeysController {
    cosmos_keys_service: Arc<CosmosKeysService>,
}

impl CosmosKeysController {
    pub(crate) fn new(cosmos_keys_service: Arc<CosmosKeysService>) -> Self {
        Self {
            cosmos_keys_service,
        }
    }

    pub(crate) async fn add_random(&self, alias: &str) -> IndyResult<KeyInfo> {
        trace!("add_random > alias {:?}", alias);
        let res = self.cosmos_keys_service.add_random(&alias).await;
        trace!("add_random < {:?}", res);
        res
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
        alias: &str,
        mnemonic: &str,
    ) -> IndyResult<KeyInfo> {
        trace!("add_from_mnemonic > alias {:?}", alias);
        let res = self
            .cosmos_keys_service
            .add_from_mnemonic(&alias, mnemonic)
            .await?;
        trace!("add_from_mnemonic < {:?}", res);
        Ok(res)
    }

    pub(crate) async fn key_info(&self, alias: &str) -> IndyResult<KeyInfo> {
        trace!("key_info > alias {:?}", alias);
        let res = self.cosmos_keys_service.key_info(&alias).await?;
        trace!("key_info < {:?}", res);
        Ok(res)
    }

    pub(crate) async fn sign(&self, alias: &str, tx: &[u8]) -> IndyResult<Vec<u8>> {
        trace!("key_info > alias {:?}, tx {:?}", alias, tx);

        let sign_doc = SignDoc::from_bytes(tx)?;
        let signed = self.cosmos_keys_service.sign(alias, sign_doc).await?;
        let signed = signed.to_bytes()?;

        trace!("key_info < signed {:?}", signed);

        Ok(signed)
    }
}
