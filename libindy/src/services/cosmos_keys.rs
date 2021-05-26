//! Service to manage Cosmos keys

use crate::domain::cosmos_keys::KeyInfo;
use cosmos_sdk::crypto::secp256k1::signing_key::Secp256k1Signer;
use cosmos_sdk::crypto::secp256k1::SigningKey as CosmosSigningKey;
use cosmos_sdk::tx::{Raw, SignDoc};
use futures::lock::Mutex as MutexF;
use indy_api_types::{
    errors::{IndyErrorKind, IndyResult},
    IndyError,
};
use k256::ecdsa::signature::rand_core::OsRng;
use k256::ecdsa::SigningKey;
use rand::rngs::StdRng;
use rand_seeder::Seeder;
use rust_base58::ToBase58;
use std::collections::HashMap;

pub(crate) struct CosmosKeysService {
    // TODO: Persistence
    // key alias -> SEC1-encoded secp256k1 ECDSA private key
    keys: MutexF<HashMap<String, Vec<u8>>>,
}

impl CosmosKeysService {
    pub(crate) fn new() -> Self {
        Self {
            keys: MutexF::new(HashMap::new()),
        }
    }

    async fn set_signing_key_bytes(&self, alias: &str, bytes: &[u8]) -> IndyResult<()> {
        let mut keys = self.keys.lock().await;

        if keys.contains_key(alias) {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                "Key already exists",
            ));
        }

        keys.insert(alias.to_owned(), bytes.to_vec());

        Ok(())
    }

    async fn get_signing_key_bytes(&self, alias: &str) -> IndyResult<Vec<u8>> {
        let keys = self.keys.lock().await;

        let bytes = keys.get(alias).ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Key not found",
        ))?;

        Ok(bytes.clone())
    }

    async fn set_signing_key(&self, alias: &str, key: &SigningKey) -> IndyResult<()> {
        let bytes = key.to_bytes().to_vec();
        self.set_signing_key_bytes(alias, &bytes).await?;
        Ok(())
    }

    async fn get_signing_key(&self, alias: &str) -> IndyResult<SigningKey> {
        let bytes = self.get_signing_key_bytes(alias).await?;
        Ok(SigningKey::from_bytes(&bytes)?)
    }

    async fn get_cosmos_signing_key(&self, alias: &str) -> IndyResult<CosmosSigningKey> {
        let key = self.get_signing_key(alias).await?;
        Ok(CosmosSigningKey::from(
            Box::new(key) as Box<dyn Secp256k1Signer>
        ))
    }

    pub(crate) async fn add_random(&self, alias: &str) -> IndyResult<KeyInfo> {
        let key = k256::ecdsa::SigningKey::random(&mut OsRng);
        self.set_signing_key(alias, &key).await?;

        Ok(self.get_info(alias).await?)
    }

    pub(crate) async fn add_from_mnemonic(
        &self,
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
        let cosmos_keys_service = CosmosKeysService::new();

        let key_info = cosmos_keys_service.add_random("alice").await.unwrap();

        assert_eq!(key_info.alias, "alice")
    }

    #[async_std::test]
    async fn test_add_from_mnemonic() {
        let cosmos_keys_service = CosmosKeysService::new();

        let alice = cosmos_keys_service
            .add_from_mnemonic("alice", "secret phrase")
            .await
            .unwrap();

        let bob = cosmos_keys_service
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
