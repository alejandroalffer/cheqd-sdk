//! Service to manage Cosmos keys

use crate::domain::verim_keys::{KeyInfo, Key};
use cosmos_sdk::crypto::secp256k1::signing_key::Secp256k1Signer;
use cosmos_sdk::crypto::secp256k1::SigningKey as CosmosSigningKey;
use cosmos_sdk::tx::{Raw, SignDoc};
use futures::lock::Mutex as MutexF;
use indy_api_types::{errors::{IndyErrorKind, IndyResult}, IndyError, WalletHandle};
use k256::ecdsa::signature::rand_core::OsRng;
use k256::ecdsa::SigningKey;
use rand::rngs::StdRng;
use rand_seeder::Seeder;
use rust_base58::ToBase58;
use std::collections::HashMap;
use indy_wallet::{WalletService, RecordOptions};
use failure::ResultExt;
use indy_api_types::errors::IndyResultExt;

pub(crate) struct VerimKeysService {
    wallet_service: WalletService,
}

impl VerimKeysService {
    pub(crate) fn new(wallet_service: WalletService) -> Self {
        Self {
            wallet_service
        }
    }

    async fn store_key(&self, wallet_handle: WalletHandle, key: &Key) -> IndyResult<()> {
        self.wallet_service
            .add_indy_object(wallet_handle, &key.alias, &key, &HashMap::new())
            .await
            .to_indy(IndyErrorKind::IOError, "Can't write verim key")?;

        Ok(())
    }

    async fn load_key(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<Key> {
        let key = self.wallet_service
            .get_indy_object(wallet_handle, &alias, &RecordOptions::id_value())
            .await
            .to_indy(IndyErrorKind::IOError, "Can't write verim key")?;

        Ok(key)
    }

    // TODO: Keep or remove?
    fn signing_key_to_bytes(&self, key: &SigningKey) -> Vec<u8> {
        key.to_bytes().to_vec()
    }

    // TODO: Keep or remove?
    fn bytes_to_signing_key(&self, bytes: &[u8]) -> IndyResult<SigningKey> {
        Ok(SigningKey::from_bytes(bytes)?)
    }

    // TODO: Keep or remove?
    fn bytes_to_cosmos_signing_key(&self, bytes: &[u8]) -> CosmosSigningKey {
        CosmosSigningKey::from(
            Box::new(bytes.clone()) as Box<dyn Secp256k1Signer>
        )
    }

    fn key_to_key_info(&self, key: &Key) -> IndyResult<KeyInfo> {
        let sig_key = self.bytes_to_cosmos_signing_key(&key.priv_key);
        let pub_key = sig_key.public_key();
        let account_id = pub_key.account_id("cosmos")?;

        let key_info = KeyInfo::new(
            alias.to_owned(),
            account_id.to_string(),
            pub_key.to_bytes().to_base58(),
        );

        Ok(key_info)
    }

    pub(crate) async fn add_random(&self, wallet_handle: WalletHandle, alias: &str) -> IndyResult<KeyInfo> {
        let signing_key = k256::ecdsa::SigningKey::random(&mut OsRng);
        let key = Key::new(alias.to_string(), signing_key.to_bytes().to_vec());
        self.store_key(wallet_handle, &key).await?;
        Ok(self.key_to_key_info(&key)?)
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
        wallet_handle: WalletHandle,
        alias: &str,
        mnemonic: &str,
    ) -> IndyResult<KeyInfo> {
        let mut rng: StdRng = Seeder::from(mnemonic).make_rng();
        let key = k256::ecdsa::SigningKey::random(&mut rng);
        self.set_signing_key(alias, &key).await?;

        Ok(self.get_info(alias).await?)
    }

    pub(crate) async fn get_info(&self, alias: &str) -> IndyResult<KeyInfo> {
        let key = self.get_cosmos_signing_key(alias).await?;
        let pub_key = key.public_key();
        let account_id = pub_key.account_id("cosmos")?;

        let key_info = KeyInfo::new(
            alias.to_owned(),
            account_id.to_string(),
            pub_key.to_bytes().to_base58(),
        );

        Ok(key_info)
    }

    pub(crate) async fn sign(&self, alias: &str, tx: SignDoc) -> IndyResult<Raw> {
        let key = self.get_cosmos_signing_key(alias).await?;
        Ok(tx.sign(&key)?)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use cosmos_sdk::crypto::secp256k1::signing_key::{
        Secp256k1Signer, SigningKey as CosmosSigningKey,
    };
    use k256::ecdsa::signature::Signature;
    use k256::ecdsa::signature::Signer;
    use k256::elliptic_curve::rand_core::OsRng;

    #[async_std::test]
    async fn test_add_random() {
        let verim_keys_service = VerimKeysService::new();

        let key_info = verim_keys_service.add_random("alice").await.unwrap();

        assert_eq!(key_info.alias, "alice")
    }

    #[async_std::test]
    async fn test_add_from_mnemonic() {
        let verim_keys_service = VerimKeysService::new();

        let alice = verim_keys_service
            .add_from_mnemonic("alice", "secret phrase")
            .await
            .unwrap();

        let bob = verim_keys_service
            .add_from_mnemonic("bob", "secret phrase")
            .await
            .unwrap();

        assert_eq!(alice.pub_key, bob.pub_key)
    }

    #[test]
    fn test_private_key_import_export() {
        let key = k256::ecdsa::SigningKey::random(&mut OsRng);
        let bytes = key.to_bytes().to_vec();
        let imported = k256::ecdsa::SigningKey::from_bytes(&bytes).unwrap();

        let msg = vec![251u8, 252, 253, 254];

        let s1: k256::ecdsa::Signature = key.sign(&msg);
        let s2: k256::ecdsa::Signature = imported.sign(&msg);

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_private_key_compatibility() {
        let msg = vec![251u8, 252, 253, 254];

        let key = k256::ecdsa::SigningKey::random(&mut OsRng);
        let s1: k256::ecdsa::Signature = key.sign(&msg);
        let s1 = s1.as_ref().to_vec();

        let cosmos_key = CosmosSigningKey::from(Box::new(key) as Box<dyn Secp256k1Signer>);
        let s2 = cosmos_key.sign(&msg).unwrap().as_bytes().to_vec();

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_pub_key_compatibility() {
        let key = k256::ecdsa::SigningKey::random(&mut OsRng);
        let pub_key = key.verify_key().to_bytes().to_vec();

        let cosmos_key = CosmosSigningKey::from(Box::new(key) as Box<dyn Secp256k1Signer>);
        let cosmos_pub_key = cosmos_key.public_key().to_bytes();

        assert_eq!(pub_key, cosmos_pub_key);
    }
}
