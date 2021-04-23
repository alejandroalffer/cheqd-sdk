//! Service to manage Cosmos keys

use cosmos_sdk::crypto::secp256k1::SigningKey;
use futures::lock::Mutex as MutexF;
use indy_api_types::{
    errors::{IndyErrorKind, IndyResult},
    IndyError,
};
use rand::Rng;
use rust_base58::ToBase58;
use std::collections::HashMap;

pub struct KeyInfo {
    pub alias: String,
    pub account_id: String,
    pub pub_key: String,
}

impl KeyInfo {
    pub fn new(alias: String, account_id: String, pub_key: String) -> Self {
        KeyInfo {
            alias,
            account_id,
            pub_key,
        }
    }
}

pub struct KeysService {
    seeds: MutexF<HashMap<String, Vec<u8>>>,
}

impl KeysService {
    pub fn new() -> Self {
        Self {
            seeds: MutexF::new(HashMap::new()),
        }
    }

    async fn add_random(&self, alias: &str) -> IndyResult<()> {
        let mut seeds = self.seeds.lock().await;

        if seeds.contains_key(alias) {
            return Err(IndyError::from_msg(
                IndyErrorKind::InvalidState,
                "Key already exists",
            ));
        }

        let seed = rand::thread_rng().gen::<[u8; 32]>();
        seeds.insert(alias.to_owned(), seed.to_vec());

        Ok(())
    }

    pub fn add_from_mnemonic(&self, _alias: String, _mnemonic: &str) -> IndyResult<()> {
        unimplemented!()
    }

    async fn key_info(&self, alias: &str) -> IndyResult<KeyInfo> {
        let seeds = self.seeds.lock().await;
        let seed = seeds.get(alias).ok_or(IndyError::from_msg(
            IndyErrorKind::InvalidState,
            "Key not found",
        ))?;

        let key = SigningKey::from_bytes(seed)?;
        let pub_key = key.public_key();
        let account_id = pub_key.account_id("cosmos")?;

        let key_info = KeyInfo::new(
            alias.to_owned(),
            account_id.to_string(),
            pub_key.to_bytes().to_base58(),
        );

        Ok(key_info)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_add_random() {
        let keys_service = KeysService::new();
        keys_service.add_random("alice").await.unwrap();
    }

    #[async_std::test]
    async fn test_key_info() {
        let keys_service = KeysService::new();

        keys_service.add_random("alice").await.unwrap();
        let key_info = keys_service.key_info("alice").await.unwrap();

        assert_eq!(key_info.alias, "alice")
    }
}
