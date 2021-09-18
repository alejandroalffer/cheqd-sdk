//! Cosmos key management service

use std::collections::HashMap;

use async_std::sync::Arc;
use cosmrs::tx::SignDoc;
use indy_api_types::errors::{IndyErrorKind, err_msg, IndyResult, IndyResultExt};
use indy_api_types::WalletHandle;
use indy_wallet::{RecordOptions, SearchOptions};

use crate::domain::cheqd_keys::{Key, KeyInfo};
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
            .to_indy(IndyErrorKind::WalletItemAlreadyExists, "Can't write cheqd key")?;

        Ok(())
    }

    async fn load_key(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<Key> {
        let key = self.wallet_service
            .get_indy_object(wallet_handle, &alias, &RecordOptions::id_value())
            .await
            .to_indy(IndyErrorKind::WalletItemNotFound, "Can't read cheqd key")?;

        Ok(key)
    }

    pub(crate) async fn add_random(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<String> {
        trace!("add_random > alias {:?}", alias);
        let key = self.cheqd_keys_service.new_random(&alias)?;
        self.store_key(wallet_handle, &key).await?;
        let key_info = self.cheqd_keys_service.get_info(&key)?;
        let key_info = serde_json::to_string(&key_info).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize structure KeyInfo"
        )?;
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
        let key_info = serde_json::to_string(&key_info).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize structure KeyInfo"
        )?;
        trace!("add_from_mnemonic < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn get_info(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<String> {
        trace!("get_info > alias {:?}", alias);
        let key = self.load_key(wallet_handle, alias).await?;
        let key_info = self.cheqd_keys_service.get_info(&key)?;
        let key_info = serde_json::to_string(&key_info).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize structure KeyInfo"
        )?;
        trace!("get_info < {:?}", key_info);
        Ok(key_info)
    }

    pub(crate) async fn get_list_keys(&self, wallet_handle: WalletHandle) -> IndyResult<String> {
        trace!("get_list_keys >");

        let mut key_search = self
            .wallet_service
            .search_indy_records::<Key>(wallet_handle, "{}", &SearchOptions::id_value())
            .await?;

        let mut keys: Vec<KeyInfo> = Vec::new();

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

            let key_info = self.cheqd_keys_service.get_info(&key)?;
            keys.push(key_info);
        }

        let result = serde_json::to_string(&keys).to_indy(
            IndyErrorKind::InvalidState,
            "Cannot serialize structure KeyInfo"
        )?;

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

#[cfg(test)]
mod tests {
    use indy_api_types::errors::IndyErrorKind;
    use crate::controllers::{CheqdKeysController, WalletController};
    use crate::services::{CheqdKeysService, WalletService, CryptoService};
    use rand::{distributions::Alphanumeric, Rng};
    use crate::domain::cheqd_keys::Key;
    use async_std::sync::Arc;
    use indy_api_types::{
        domain::wallet::{Config, Credentials, KeyDerivationMethod}
    };
    use failure::AsFail;

    #[async_std::test]
    async fn wallet_item_not_found() {
        let cheqd_keys_service = CheqdKeysService::new();
        let wallet_service = WalletService::new();
        let cheqd_controller = CheqdKeysController::new(Arc::from(cheqd_keys_service),
                                                        Arc::from(wallet_service));
        let wallet_controller = WalletController::new(Arc::from(WalletService::new()), Arc::new(CryptoService::new()));

        let mut wallet_config = Config {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
            storage_type: None,
            storage_config: None,
            cache: None
        };
        let mut wallet_cred = Credentials {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::RAW
        };
        wallet_controller.create(
            wallet_config.clone(),
            wallet_cred.clone())
            .await
            .unwrap();

        let wallet_handle = wallet_controller.open(wallet_config, wallet_cred)
            .await
            .unwrap();

        let err =cheqd_controller.load_key(
            wallet_handle,
        "test_key_which_is_absent")
            .await
            .unwrap_err();
        assert!(err.to_string().contains(IndyErrorKind::WalletItemNotFound.as_fail().to_string().as_str()));
    }

    #[async_std::test]
    async fn wallet_already_exists_on_store_key() {
        let cheqd_keys_service = CheqdKeysService::new();
        let wallet_service = WalletService::new();
        let cheqd_controller = CheqdKeysController::new(Arc::from(cheqd_keys_service),
                                                        Arc::from(wallet_service));
        let wallet_controller = WalletController::new(Arc::from(WalletService::new()), Arc::new(CryptoService::new()));

        let mut wallet_config = Config {
            id: rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(7)
                .map(char::from)
                .collect(),
            storage_type: None,
            storage_config: None,
            cache: None
        };
        let mut wallet_cred = Credentials {
            key: "6nxtSiXFvBd593Y2DCed2dYvRY1PGK9WMtxCBjLzKgbw".to_string(),
            rekey: None,
            storage_credentials: None,
            key_derivation_method: KeyDerivationMethod::RAW,
            rekey_derivation_method: KeyDerivationMethod::RAW
        };
        wallet_controller.create(
            wallet_config.clone(),
            wallet_cred.clone())
            .await
            .unwrap();

        let wallet_handle = wallet_controller.open(wallet_config, wallet_cred)
            .await
            .unwrap();

        let key = Key::new(
            "test_alias".to_string(),
            "3SeuRm3uYuQDYmHeuMLu1xNHozNTtzS3kbZRFMMCWrX4".to_string().into_bytes());
        let res = cheqd_controller.store_key(wallet_handle,
                                             &key)
            .await;
        let err = cheqd_controller.store_key(wallet_handle,
                                   &key)
            .await
            .unwrap_err();
        assert!(err.to_string().contains(IndyErrorKind::WalletItemAlreadyExists.as_fail().to_string().as_str()));
    }
}
