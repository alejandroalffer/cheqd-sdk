//! Service to manage Cosmos keys

use cosmrs::crypto::secp256k1::EcdsaSigner;
use cosmrs::crypto::secp256k1::SigningKey as CosmosSigningKey;
use cosmrs::tx::{Raw, SignDoc};
use indy_api_types::errors::{IndyResult, IndyResultExt, IndyErrorKind};
use k256::ecdsa::signature::rand_core::OsRng;
use k256::ecdsa::SigningKey;
use rand::rngs::StdRng;
use rust_base58::ToBase58;

use crate::domain::cheqd_keys::{Key, KeyInfo};
use sha3::Digest;
use rand::SeedableRng;

pub(crate) struct CheqdKeysService {}

impl CheqdKeysService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    fn bytes_to_signing_key(bytes: &[u8]) -> IndyResult<SigningKey> {
        Ok(SigningKey::from_bytes(bytes).to_indy(
                IndyErrorKind::InvalidStructure,
                "Error was raised while converting bytes of key into the k256::ecdsa::SigningKey object"
            )?)
    }

    fn bytes_to_cosmos_signing_key(bytes: &[u8]) -> IndyResult<CosmosSigningKey> {
        let sig_key = Self::bytes_to_signing_key(bytes)?;
        Ok(CosmosSigningKey::from(
            Box::new(sig_key) as Box<dyn EcdsaSigner>
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
        let seed = sha3::Sha3_256::digest(&mnemonic.as_bytes());
        let mut rng = StdRng::from_seed(seed.into());
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
    use cosmrs::crypto::secp256k1::{
        EcdsaSigner, SigningKey as CosmosSigningKey,
    };
    use k256::ecdsa::signature::Signature;
    use k256::ecdsa::signature::Signer;
    use k256::elliptic_curve::rand_core::OsRng;

    use super::*;

    #[async_std::test]
    async fn test_add_random() {
        let cheqd_keys_service = CheqdKeysService::new();

        let key = cheqd_keys_service.new_random("alice").unwrap();

        assert_eq!(key.alias, "alice")
    }

    #[async_std::test]
    async fn test_add_from_mnemonic() {
        let cheqd_keys_service = CheqdKeysService::new();

        let alice = cheqd_keys_service
            .new_from_mnemonic("alice", "secret phrase")
            .unwrap();
        let alice_info = cheqd_keys_service.get_info(&alice).unwrap();

        let bob = cheqd_keys_service
            .new_from_mnemonic("bob", "secret phrase")
            .unwrap();
        let bob_info = cheqd_keys_service.get_info(&bob).unwrap();

        assert_eq!(alice_info.pub_key, bob_info.pub_key)
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

        let cosmos_key = CosmosSigningKey::from(Box::new(key) as Box<dyn EcdsaSigner>);
        let s2 = cosmos_key.sign(&msg).unwrap().as_bytes().to_vec();

        assert_eq!(s1, s2);
    }

    #[test]
    fn test_pub_key_compatibility() {
        let key = k256::ecdsa::SigningKey::random(&mut OsRng);
        let pub_key = key.verify_key().to_bytes().to_vec();

        let cosmos_key = CosmosSigningKey::from(Box::new(key) as Box<dyn EcdsaSigner>);
        let cosmos_pub_key = cosmos_key.public_key().to_bytes();

        assert_eq!(pub_key, cosmos_pub_key);
    }
}
