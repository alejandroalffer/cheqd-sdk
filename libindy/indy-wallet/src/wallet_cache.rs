use lru::LruCache;
use crate::wallet::EncryptedValue;
use crate::storage::{StorageRecord, Tag, TagName};
use crate::RecordOptions;
use std::sync::Mutex;
use indy_api_types::domain::wallet::CacheConfig;
use std::collections::HashSet;
use std::iter::FromIterator;
use crate::storage::Tag::{Encrypted, PlainText};
use crate::storage::TagName::{OfEncrypted, OfPlain};

#[derive(PartialEq, Eq, Hash)]
pub struct WalletCacheKey {
    type_: Vec<u8>,
    id: Vec<u8>,
}

pub struct WalletCacheValue {
    value: EncryptedValue,
    tags: Vec<Tag>,
}

pub struct WalletCache {
    cache: Option<Mutex<LruCache<WalletCacheKey, WalletCacheValue>>>,
    cache_entities: HashSet<String>,
}

impl WalletCache {
    pub fn new(config: Option<CacheConfig>) -> Self {
        match config {
            Some(cache_config) if cache_config.size.unwrap_or(10) > 0 && !cache_config.entities.is_empty() => {
                WalletCache {
                    cache: Some(Mutex::new(LruCache::new(cache_config.size.unwrap_or(10)))),
                    cache_entities: HashSet::from_iter(cache_config.entities.iter().cloned()),
                }
            }
            _ => {
                WalletCache { // no cache
                    cache: None,
                    cache_entities: HashSet::new(),
                }
            }
        }
    }

    pub fn is_type_cacheable(&self, type_: &str) -> bool {
        self.cache.is_some() && self.cache_entities.contains(&type_.to_owned())
    }

    pub fn add(
        &self,
        type_: &str,
        etype: &[u8],
        eid: &[u8],
        evalue: &EncryptedValue,
        etags: &[Tag],
    ) {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let value = WalletCacheValue {
                    value: evalue.to_owned(),
                    tags: etags.to_owned(),
                };
                let _ = protected_cache.lock().map(|mut cache|{cache.put(key, value)});
            }
        }
    }

    pub fn add_tags(
        &self,
        type_: &str,
        etype: &[u8],
        eid: &[u8],
        etags: &[Tag],
    ) {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let _ = protected_cache.lock().map(|mut cache|{
                    let _ = cache.get_mut(&key).map(|v|{
                        v.tags.append(&mut etags.to_owned())
                    });
                });
            }
        }
    }

    pub fn update_tags(
        &self,
        type_: &str,
        etype: &[u8],
        eid: &[u8],
        etags: &[Tag],
    ) {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let _ = protected_cache.lock().map(|mut cache|{
                    let _ = cache.get_mut(&key).map(|v|{
                        v.tags = etags.to_vec()
                    });
                });
            }
        }
    }

    pub fn delete_tags(
        &self,
        type_: &str,
        etype: &[u8],
        eid: &[u8],
        etag_names: &[TagName],
    ) {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let mut enc_tag_names = HashSet::new();
                let mut plain_tag_names = HashSet::new();
                for x in etag_names {
                    match x {
                        OfEncrypted(value) => enc_tag_names.insert(value),
                        OfPlain(value) => plain_tag_names.insert(value),
                    };
                }
                let _ = protected_cache.lock().map(|mut cache|{
                    let _ = cache.get_mut(&key).map(|v|{
                        v.tags.retain(|el| {
                            match el {
                                Encrypted(tag_name, _) => {
                                    !enc_tag_names.contains(tag_name)
                                },
                                PlainText(tag_name, _) => {
                                    !plain_tag_names.contains(tag_name)
                                }
                            }
                        })
                    });
                });
            }
        }
    }

    pub fn update(
        &self,
        type_: &str,
        etype: &[u8],
        eid: &[u8],
        evalue: &EncryptedValue,
    ) {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let _ = protected_cache.lock().map(|mut cache|{
                    let _ = cache.get_mut(&key).map(|v|{
                        v.value = evalue.to_owned()
                    });
                });
            }
        }
    }

    pub fn get(
        &self,
        type_: &str,
        etype: &[u8],
        eid: &[u8],
        options: &RecordOptions
    ) -> Option<StorageRecord> {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let mut cache = protected_cache.lock().unwrap();
                (*cache).get(&key).map(|v|{
                    StorageRecord {
                        id: eid.to_owned(),
                        value: if options.retrieve_value {Some(v.value.clone())} else {None},
                        type_: if options.retrieve_type {Some(etype.to_owned())} else {None},
                        tags: if options.retrieve_tags {Some(v.tags.clone())} else {None},
                    }
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn delete(&self, type_: &str, etype: &[u8], eid: &[u8]) {
        if let Some(protected_cache) = &self.cache {
            if self.cache_entities.contains(&type_.to_owned()) {
                let key = WalletCacheKey {
                    type_: etype.to_owned(),
                    id: eid.to_owned(),
                };
                let _ = protected_cache.lock().map(|mut cache|{
                    cache.pop(&key)
                });
            }
        }
    }
}