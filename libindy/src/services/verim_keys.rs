//! Service to manage Cosmos keys

use cosmos_sdk::crypto::secp256k1::signing_key::Secp256k1Signer;
use cosmos_sdk::crypto::secp256k1::SigningKey as CosmosSigningKey;
use cosmos_sdk::tx::{Raw, SignDoc};
use indy_api_types::errors::IndyResult;
use k256::ecdsa::signature::rand_core::OsRng;
use k256::ecdsa::SigningKey;
use rand::rngs::StdRng;
use rand_seeder::Seeder;
use rust_base58::ToBase58;

use crate::domain::verim_keys::{Key, KeyInfo};

pub(crate) struct VerimKeysService {}

impl VerimKeysService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn signing_key_to_bytes(&self, key: &SigningKey) -> Vec<u8> {
        key.to_bytes().to_vec()
    }

    fn bytes_to_signing_key(bytes: &[u8]) -> IndyResult<SigningKey> {
        Ok(SigningKey::from_bytes(bytes)?)
    }

    fn bytes_to_cosmos_signing_key(bytes: &[u8]) -> IndyResult<CosmosSigningKey> {
        let sig_key = Self::bytes_to_signing_key(bytes)?;
        Ok(CosmosSigningKey::from(
            Box::new(sig_key) as Box<dyn Secp256k1Signer>
        ))
    }

    pub(crate) fn new_random(&self, alias: &str) -> IndyResult<Key> {
        let sig_key = k256::ecdsa::SigningKey::random(&mut OsRng);
        let key = Key::new(alias.to_string(), sig_key.to_bytes().to_vec());
        Ok(key)
    }

    pub(crate) fn new_from_mnemonic(
        &self,
        alias: &str,
        mnemonic: &str,
    ) -> IndyResult<Key> {
        let mut rng: StdRng = Seeder::from(mnemonic).make_rng();
        let sig_key = k256::ecdsa::SigningKey::random(&mut rng);
        let key = Key::new(alias.to_string(), sig_key.to_bytes().to_vec());
        Ok(key)
    }

    pub(crate) fn get_info(&self, key: &Key) -> IndyResult<KeyInfo> {
        let sig_key = Self::bytes_to_cosmos_signing_key(&key.priv_key)?;
        let pub_key = sig_key.public_key();
        let account_id = pub_key.account_id("cosmos")?;

        let key_info = KeyInfo::new(
            key.alias.to_owned(),
            account_id.to_string(),
            pub_key.to_bytes().to_base58(),
        );

        Ok(key_info)
    }

    pub(crate) async fn sign(&self, key: &Key, tx: SignDoc) -> IndyResult<Raw> {
        let sig_key = Self::bytes_to_cosmos_signing_key(&key.priv_key)?;
        Ok(tx.sign(&sig_key)?)
    }
}

#[cfg(test)]
mod test {
    use cosmos_sdk::crypto::secp256k1::signing_key::{
        Secp256k1Signer, SigningKey as CosmosSigningKey,
    };
    use k256::ecdsa::signature::Signature;
    use k256::ecdsa::signature::Signer;
    use k256::elliptic_curve::rand_core::OsRng;

    use super::*;

    #[async_std::test]
    async fn test_add_random() {
        let verim_keys_service = VerimKeysService::new();

        let key = verim_keys_service.new_random("alice").await.unwrap();

        assert_eq!(key.alias, "alice")
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
