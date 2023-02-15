use std::{collections::HashMap, hash::Hash, path::PathBuf};

pub mod codec;
pub mod config;
pub mod error;
pub mod store;

use error::result::KvsError;
use store::Store;

pub type Result<T> = std::result::Result<T, KvsError>;


pub struct KvStore {
    // 实际的存储引擎
    store: Store,
}

impl KvStore {
    pub fn open(path: impl Into<PathBuf>) -> Result<Self> {
        let store = Store::new();
        Ok(Self { store })
    }

    pub fn set(&mut self, key: String, value: String) -> Result<()> {
        panic!();
        // self.store.insert(key, value)
    }

    pub fn get(&self, key: String) -> Result<Option<String>> {
        panic!();
        // self.store.get(&key).cloned()
    }

    pub fn remove(&mut self, key: String) -> Result<()> {
        panic!();
        // self.store.remove(&key)
    }
}
