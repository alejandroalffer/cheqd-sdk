//! Cosmos key management service

use std::collections::HashMap;

use async_std::sync::Arc;
use cosmos_sdk::tx::SignDoc;
use indy_api_types::errors::{IndyErrorKind, err_msg, IndyResult, IndyResultExt};
use indy_api_types::WalletHandle;
use indy_wallet::{RecordOptions, SearchOptions};

use crate::domain::cheqd_keys::Key;
use crate::domain::cheqd_ledger::cosmos_ext::CosmosSignDocExt;
use crate::services::{CheqdKeysService, WalletService};

pub(crate) struct CheqdKeysController {
    cheqd_keys_service: Arc<CheqdKeysService>,
    wallet_service: Arc<WalletService>,
}

impl CheqdKeysController {
    pub(crate) fn new(cheqd_keys_service: Arc<CheqdKeysService>, wallet_service: Arc<WalletService>) -> Self {
        Self {
            cheqd_keys_service,
            wallet_service,
        }
    }

    async fn store_key(&self, wallet_handle: WalletHandle, key: &Key) -> IndyResult<()> {
        self.wallet_service
            .add_indy_object(wallet_handle, &key.alias, &key, &HashMap::new())
            .await
            .to_indy(IndyErrorKind::IOError, "Can't write cheqd key")?;

        Ok(())
    }

    async fn load_key(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<Key> {
        let key = self.wallet_service
            .get_indy_object(wallet_handle, &alias, &RecordOptions::id_value())
            .await
            .to_indy(IndyErrorKind::IOError, "Can't read cheqd key")?;

        Ok(key)
    }

    pub(crate) async fn add_random(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<String> {
        trace!("add_random > alias {:?}", alias);
        let key = self.cheqd_keys_service.new_random(&alias)?;
        self.store_key(wallet_handle, &key).await?;
        let key_info = self.cheqd_keys_service.get_info(&key)?;
        let key_info = serde_json::to_string(&key_info)?;
        trace!("add_random < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
        wallet_handle: WalletHandle,
        alias: &str,
        mnemonic: &str,
    ) -> IndyResult<String> {
        trace!("add_from_mnemonic > alias {:?}", alias);
        let key = self
            .cheqd_keys_service
            .new_from_mnemonic(&alias, mnemonic)?;
        self.store_key(wallet_handle, &key).await?;
        let key_info = self.cheqd_keys_service.get_info(&key)?;
        let key_info = serde_json::to_string(&key_info)?;
        trace!("add_from_mnemonic < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn get_info(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<String> {
        trace!("get_info > alias {:?}", alias);
        let key = self.load_key(wallet_handle, alias).await?;
        let key_info = self.cheqd_keys_service.get_info(&key)?;
        let key_info = serde_json::to_string(&key_info)?;
        trace!("get_info < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn get_list_keys(&self, wallet_handle: WalletHandle) -> IndyResult<String> {
        trace!("get_list_keys >");

        let mut key_search = self
            .wallet_service
            .search_indy_records::<Key>(wallet_handle, "{}", &SearchOptions::id_value())
            .await?;

        let mut keys: Vec<Key> = Vec::new();

        while let Some(key_record) = key_search.fetch_next_record().await? {
            let key_id = key_record.get_id();

            let key: Key = key_record
                .get_value()
                .ok_or_else(|| err_msg(IndyErrorKind::InvalidState, "No value for Key record"))
                .and_then(|tags_json| {
                    serde_json::from_str(&tags_json).to_indy(
                        IndyErrorKind::InvalidState,
                        format!("Cannot deserialize Key {:?}", key_id),
                    )
                })?;

            keys.push(key);
        }

        let result = serde_json::to_string(&keys)?;

        trace!("get_list_keys < {:?}", result);

        Ok(result)
    }

    pub(crate) async fn sign(&self, wallet_handle: WalletHandle, alias: &str, tx: &[u8]) -> IndyResult<Vec<u8>> {
        trace!("sign > alias {:?}, tx {:?}", alias, tx);

        let sign_doc = SignDoc::from_bytes(tx)?;

        let key = self.load_key(wallet_handle, alias).await?;
        let signed = self.cheqd_keys_service.sign(&key, sign_doc).await?;
        let signed = signed.to_bytes()?;

        trace!("sign < signed {:?}", signed);

        Ok(signed)
    }
}
