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

const DEFAULT_CACHE_SIZE: usize = 10;

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
            Some(cache_config) if cache_config.size.unwrap_or(DEFAULT_CACHE_SIZE) > 0 && !cache_config.entities.is_empty() => {
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

#[cfg(test)]
mod tests {
    use super::*;

    const TYPE_A: &str = "TypeA";
    const TYPE_B: &str = "TypeB";
    const TYPE_NON_CACHED: &str = "TypeNonCached";

    const ETYPE1: Vec<u8> = vec![1, 2, 3, 1];
    const ETYPE2: Vec<u8> = vec![1, 2, 3, 2];
    const EID1: Vec<u8> = vec![2, 3, 4, 1];
    const EID2: Vec<u8> = vec![2, 3, 4, 2];
    const EVALUE1: EncryptedValue = EncryptedValue {
        data: vec![3, 4, 5, 6, 7, 1],
        key: vec![4, 5, 6, 7, 8, 9, 1]
    };
    const EVALUE2: EncryptedValue = EncryptedValue {
        data: vec![3, 4, 5, 6, 7, 2],
        key: vec![4, 5, 6, 7, 8, 9, 2]
    };
    const ETAG_E1: Tag = Tag::Encrypted(vec![5, 6, 7, 1], vec![10, 11, 12, 1]);
    const ETAG_E2: Tag = Tag::Encrypted(vec![5, 6, 7, 2], vec![10, 11, 12, 2]);
    const ETAG_P1: Tag = Tag::PlainText(vec![6, 7, 8, 1], "PlainTag1".to_string());
    const ETAG_P2: Tag = Tag::PlainText(vec![6, 7, 8, 2], "PlainTag2".to_string());

    const FULL_OPTIONS: RecordOptions = RecordOptions {
        retrieve_type: true,
        retrieve_value: true,
        retrieve_tags: true
    };

    fn _cache() -> WalletCache {
        let config = CacheConfig {
            size: None,
            entities: vec![TYPE_A.to_string(), TYPE_B.to_string()],
            algorithm: None
        };
        WalletCache::new(Some(config))
    }

    fn _no_cache() -> WalletCache {
        let config = CacheConfig {
            size: None,
            entities: vec![],
            algorithm: None
        };
        WalletCache::new(Some(config))
    }

    fn _vec_to_hash_set(items: &[&str]) -> HashSet<String> {
        HashSet::from_iter(items.into_iter().map(|el|el.to_string()))
    }

    fn _tag_names(tags: &[Tag]) -> Vec<TagName> {
        tags.into_iter().map(|el|{
            match el {
                Encrypted(key, _) => TagName::OfEncrypted(key.to_owned()),
                PlainText(key, _) => TagName::OfPlain(key.to_owned()),
            }
        }).collect()
    }

    #[test]
    fn new_with_no_config_works() {
        let cache = WalletCache::new(None);
        assert!(cache.cache.is_none());
        assert_eq!(cache.cache_entities.len(), 0);
    }

    #[test]
    fn new_with_default_config_works() {
        let config = CacheConfig {
            size: None,
            entities: vec![],
            algorithm: None
        };
        let cache = WalletCache::new(Some(config));
        assert!(cache.cache.is_none());
        assert_eq!(cache.cache_entities.len(), 0);
    }

    #[test]
    fn new_with_size_but_no_entities_in_config_works() {
        let config = CacheConfig {
            size: Some(20),
            entities: vec![],
            algorithm: None
        };
        let cache = WalletCache::new(Some(config));
        assert!(cache.cache.is_none());
        assert_eq!(cache.cache_entities.len(), 0);
    }

    #[test]
    fn new_with_default_size_in_config_works() {
        let config = CacheConfig {
            size: None,
            entities: vec![TYPE_A.to_string(), TYPE_B.to_string()],
            algorithm: None
        };
        let cache = WalletCache::new(Some(config));
        assert!(cache.cache.is_some());
        assert_eq!(cache.cache.unwrap().get_mut().unwrap().cap(), DEFAULT_CACHE_SIZE);
        assert_eq!(cache.cache.unwrap().get_mut().unwrap().len(), 0);
        assert_eq!(cache.cache_entities.len(), 2);
        assert_eq!(cache.cache_entities, _vec_to_hash_set(&[TYPE_A, TYPE_B]));
    }

    #[test]
    fn new_with_size_in_config_works() {
        let config = CacheConfig {
            size: Some(20),
            entities: vec![TYPE_A.to_string(), TYPE_B.to_string()],
            algorithm: None
        };
        let cache = WalletCache::new(Some(config));
        assert!(cache.cache.is_some());
        assert_eq!(cache.cache.unwrap().get_mut().unwrap().cap(), 20);
        assert_eq!(cache.cache.unwrap().get_mut().unwrap().len(), 0);
        assert_eq!(cache.cache_entities.len(), 2);
        assert_eq!(cache.cache_entities, _vec_to_hash_set(&[TYPE_A, TYPE_B]));
    }

    #[test]
    fn is_type_cacheable_works() {
        let cache = _cache();
        let result = cache.is_type_cacheable(TYPE_A);
        assert_eq!(result, true);
    }

    #[test]
    fn is_type_cacheable_for_noncacheable_type_works() {
        let cache = _cache();
        let result = cache.is_type_cacheable(TYPE_NON_CACHED);
        assert_eq!(result, false);
    }

    #[test]
    fn is_type_cacheable_for_no_cache_enabled_works() {
        let cache = _no_cache();
        let result = cache.is_type_cacheable(TYPE_A);
        assert_eq!(result, false);
    }

    #[test]
    fn add_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);
    }

    #[test]
    fn add_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![]);
    }

    #[test]
    fn add_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn add_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        assert!(cache.cache.is_none());
    }

    #[test]
    fn add_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.add_tags(TYPE_A, &ETYPE1, &EID1, &[ETAG_E2]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1, ETAG_E2]);
    }

    #[test]
    fn add_tags_on_item_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        cache.add_tags(TYPE_A, &ETYPE1, &EID1, &[ETAG_E2]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E2]);
    }

    #[test]
    fn add_tags_on_non_cached_item_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.add_tags(TYPE_A, &ETYPE1, &EID2, &[ETAG_E2]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);

        let key2 = WalletCacheKey {
            type_: ETYPE1,
            id: EID2
        };

        assert!(lru.peek(&key2).is_none());
    }

    #[test]
    fn add_tags_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.add_tags(TYPE_NON_CACHED, &ETYPE1, &EID1, &[ETAG_E2]);

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn add_tags_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.add_tags(TYPE_A, &ETYPE1, &EID1, &[ETAG_E2]);

        assert!(cache.cache.is_none());
    }

    #[test]
    fn update_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update_tags(TYPE_A, &ETYPE1, &EID1, &[ETAG_E2]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E2]);
    }

    #[test]
    fn update_tags_on_item_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        cache.update_tags(TYPE_A, &ETYPE1, &EID1, &[ETAG_E2]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E2]);
    }

    #[test]
    fn update_tags_on_non_cached_item_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update_tags(TYPE_A, &ETYPE1, &EID2, &[ETAG_E2]);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);

        let key2 = WalletCacheKey {
            type_: ETYPE1,
            id: EID2
        };

        assert!(lru.peek(&key2).is_none());
    }

    #[test]
    fn update_tags_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update_tags(TYPE_NON_CACHED, &ETYPE1, &EID1, &[ETAG_E2]);

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn update_tags_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update_tags(TYPE_A, &ETYPE1, &EID1, &[ETAG_E2]);

        assert!(cache.cache.is_none());
    }

    #[test]
    fn delete_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete_tags(TYPE_A, &ETYPE1, &EID1, &_tag_names(&[ETAG_P1, ETAG_E2]));

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1]);
    }

    #[test]
    fn delete_tags_on_item_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        cache.delete_tags(TYPE_A, &ETYPE1, &EID1, &_tag_names(&[ETAG_E1]));

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![]);
    }

    #[test]
    fn delete_tags_on_non_cached_item_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete_tags(TYPE_A, &ETYPE1, &EID2, &_tag_names(&[ETAG_E2]));

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);

        let key2 = WalletCacheKey {
            type_: ETYPE1,
            id: EID2
        };

        assert!(lru.peek(&key2).is_none());
    }

    #[test]
    fn delete_tags_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete_tags(TYPE_NON_CACHED, &ETYPE1, &EID1, &_tag_names(&[ETAG_E1]));

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn delete_tags_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete_tags(TYPE_A, &ETYPE1, &EID1, &_tag_names(&[ETAG_E1]));

        assert!(cache.cache.is_none());
    }

    #[test]
    fn update_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update(TYPE_A, &ETYPE1, &EID1, &EVALUE2);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE2);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);
    }

    #[test]
    fn update_on_item_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        cache.update(TYPE_A, &ETYPE1, &EID1, &EVALUE2);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE2);
        assert_eq!(value.tags, vec![]);
    }

    #[test]
    fn update_on_non_cached_item_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update(TYPE_A, &ETYPE1, &EID2, &EVALUE2);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);

        let key2 = WalletCacheKey {
            type_: ETYPE1,
            id: EID2
        };

        assert!(lru.peek(&key2).is_none());
    }

    #[test]
    fn update_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update(TYPE_A, &ETYPE1, &EID1, &EVALUE2);

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn update_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.update(TYPE_A, &ETYPE1, &EID1, &EVALUE2);

        assert!(cache.cache.is_none());
    }

    #[test]
    fn delete_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete(TYPE_A, &ETYPE1, &EID1);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
        assert!(lru.peek(&key).is_none());
    }

    #[test]
    fn delete_on_item_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        cache.delete(TYPE_A, &ETYPE1, &EID1);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
        assert!(lru.peek(&key).is_none());
    }

    #[test]
    fn delete_on_non_cached_item_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete(TYPE_A, &ETYPE1, &EID2);

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);

        let key2 = WalletCacheKey {
            type_: ETYPE1,
            id: EID2
        };

        assert!(lru.peek(&key2).is_none());
    }

    #[test]
    fn delete_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete(TYPE_NON_CACHED, &ETYPE1, &EID1);

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn delete_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        cache.delete(TYPE_A, &ETYPE1, &EID1);

        assert!(cache.cache.is_none());
    }

    #[test]
    fn get_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        let result = cache.get(TYPE_A, &ETYPE1, &EID1, &FULL_OPTIONS).unwrap();

        assert_eq!(result.id, EID1);
        assert_eq!(result.type_, Some(ETYPE1));
        assert_eq!(result.value, Some(EVALUE1));
        assert_eq!(result.tags, Some(vec![ETAG_E1, ETAG_P1]));

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![ETAG_E1, ETAG_P1]);
    }

    #[test]
    fn get_for_item_without_tags_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        let result = cache.get(TYPE_A, &ETYPE1, &EID1, &FULL_OPTIONS).unwrap();

        assert_eq!(result.id, EID1);
        assert_eq!(result.type_, Some(ETYPE1));
        assert_eq!(result.value, Some(EVALUE1));
        assert_eq!(result.tags, Some(vec![]));

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![]);
    }

    #[test]
    fn get_for_non_cached_item_works() {
        let cache = _cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[]);
        let result = cache.get(TYPE_A, &ETYPE1, &EID2, &FULL_OPTIONS);

        assert!(result.is_none());

        let key = WalletCacheKey {
            type_: ETYPE1,
            id: EID1
        };

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 1);
        let value = lru.peek(&key).unwrap();
        assert_eq!(value.value, EVALUE1);
        assert_eq!(value.tags, vec![]);
    }

    #[test]
    fn get_for_non_cacheable_type_works() {
        let cache = _cache();

        cache.add(TYPE_NON_CACHED, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        let result = cache.get(TYPE_A, &ETYPE1, &EID1, &FULL_OPTIONS);

        assert!(result.is_none());

        let lru = cache.cache.unwrap().get_mut().unwrap();
        assert_eq!(lru.len(), 0);
    }

    #[test]
    fn get_for_no_cache_enabled_works() {
        let cache = _no_cache();

        cache.add(TYPE_A, &ETYPE1, &EID1, &EVALUE1, &[ETAG_E1, ETAG_P1]);
        let result = cache.get(TYPE_A, &ETYPE1, &EID1, &FULL_OPTIONS);

        assert!(result.is_none());

        assert!(cache.cache.is_none());
    }
}