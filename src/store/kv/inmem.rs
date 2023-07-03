use std::fmt::Error;
use caches::{OnEvictCallback, RawLRU, Cache};
use crate::store::kv::entity;


pub struct InMemory {
    cap: u64,
    cache: RawLRU<String, String, EvictionCallback>,
}

impl Clone for InMemory {
    fn clone(&self) -> Self {
        let cache_cloned = RawLRU::with_on_evict_cb(self.cap as usize, EvictionCallback{}).unwrap();
        cache_cloned.iter().clone_from(&self.cache.iter());

        InMemory {
            cap: self.cap,
            cache: cache_cloned,
        }
    }
}

impl InMemory {
    pub fn new(max_size: u64) -> InMemory {
        Self {
            cap: max_size,
            cache: RawLRU::with_on_evict_cb(max_size as usize, EvictionCallback{}).unwrap(),
        }
    }
}

impl super::Storage for InMemory {
    fn put(&mut self, key: String, value:String) -> Result<bool, Error> {
        self.cache.put(key, value);
        Ok(true)
    }

    fn get(&mut self, key: String) -> Result<entity::KV, Error>{
        // &key -> is borrowing the value from key
        let res = self.cache.get( &key);

        Ok(
            entity::KV{
                key: key.to_string(),
                value: res.unwrap().to_string(),
            }
        )
    }
}

struct EvictionCallback {}

impl OnEvictCallback for EvictionCallback {
    fn on_evict<K, V>(&self, _: &K, _: &V) {}
}

