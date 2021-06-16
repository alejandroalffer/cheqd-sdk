//! Cosmos key management service

use crate::domain::verim_keys::KeyInfo;
use crate::domain::verim_ledger::cosmos_ext::CosmosSignDocExt;
use crate::services::VerimKeysService;
use async_std::sync::Arc;
use cosmos_sdk::tx::{Raw, SignDoc};
use indy_api_types::errors::{IndyErrorKind, IndyResult, IndyResultExt};
use indy_api_types::IndyError;

pub(crate) struct VerimKeysController {
    verim_keys_service: Arc<VerimKeysService>,
}

impl VerimKeysController {
    pub(crate) fn new(verim_keys_service: Arc<VerimKeysService>) -> Self {
        Self {
            verim_keys_service,
        }
    }

    pub(crate) async fn add_random(&self, alias: &str) -> IndyResult<String> {
        trace!("add_random > alias {:?}", alias);
        let res = self.verim_keys_service.add_random(&alias).await?;
        let key_info =
            serde_json::to_string(&res).to_indy(IndyErrorKind::InvalidState, "Can't add key")?;
        trace!("add_random < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
        alias: &str,
        mnemonic: &str,
    ) -> IndyResult<String> {
        trace!("add_from_mnemonic > alias {:?}", alias);
        let res = self
            .verim_keys_service
            .add_from_mnemonic(&alias, mnemonic)
            .await?;
        let key_info =
            serde_json::to_string(&res).to_indy(IndyErrorKind::InvalidState, "Can't add key")?;
        trace!("add_from_mnemonic < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn get_info(&self, alias: &str) -> IndyResult<String> {
        trace!("get_info > alias {:?}", alias);
        let res = self.verim_keys_service.get_info(&alias).await?;
        let key_info =
            serde_json::to_string(&res).to_indy(IndyErrorKind::InvalidState, "Can't add key")?;
        trace!("get_info < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn sign(&self, alias: &str, tx: &[u8]) -> IndyResult<Vec<u8>> {
        trace!("sign > alias {:?}, tx {:?}", alias, tx);

        let sign_doc = SignDoc::from_bytes(tx)?;
        let signed = self.verim_keys_service.sign(alias, sign_doc).await?;
        let signed = signed.to_bytes()?;

        trace!("sign < signed {:?}", signed);

        Ok(signed)
    }
}
