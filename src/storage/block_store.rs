use sled::Db;
use core::fmt;
use std::path::Path;
use std::fs::{File, create_dir_all};
use fs2::FileExt;
use std::error::Error;

use crate::security::crypto::CryptoEngine;

pub struct BlockStore {
    db: Db,
    crypto: CryptoEngine,
    _lock_file: File,
}

impl fmt::Debug for BlockStore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BlockStore")
            .field("db", &self.db)
            .finish()
    }
}

impl Clone for BlockStore {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            crypto: self.crypto.clone(),
            _lock_file: self._lock_file.try_clone().unwrap(),
        }
    }
}

impl BlockStore {
    pub fn new(path: impl AsRef<Path>, crypto: CryptoEngine) -> Result<Self, Box<dyn Error>> {
        if !crypto.get_secure() {
            return Err("CryptoEngine not initialized with a secure key".into());
        }

        create_dir_all(path.as_ref())?;
        if !path.as_ref().is_dir() {
            return Err("Path is not a directory".into());
        }
        let db_path = path.as_ref();
        let lock_path = db_path.join(".dess.lock");
        let lock_file = File::create(&lock_path)?;
        lock_file.try_lock_exclusive().map_err(|e| -> Box::<dyn Error>{
            format!("Failed to acquire exclusive lock on DB: {e}").into()
        })?;

        let db = sled::open(db_path)?;

        Ok(Self {
            db,
            crypto,
            _lock_file: lock_file,
        })
    }
    pub fn store_block(&self, key: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
        let encrypted = self.crypto.encrypt(data)?;
        self.db.insert(key, encrypted)?;
        self.db.flush()?;
        println!("Stored block with key: {}", key);
        Ok(())
    }
    pub fn load_block(&self, key: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        if let Some(block) = self.db.get(key)? {
            let decrypted = self.crypto.decrypt(&block)?;
            Ok(decrypted)
        } else {
            Err("Block not found".into())
        }
    }
    pub fn block_summary(&self) -> Result<(), Box<dyn Error>> {
        let mut total_size = 0;
        for item in self.db.iter() {
            let (key, value) = item?;
            total_size += value.len();
            println!("Key: {:?}, Value: {:?}", key, value);
        }
        println!("Total size: {}", total_size);
        Ok(())
    }
    pub fn delete_block(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let deleted = self.db.remove(key)?;
        self.db.flush()?; 

        if deleted.is_some() {
            println!("✅ Deleted block with key: {}", key);
            Ok(())
        } else {
            Err(format!("Block not found: {}", key).into())
        }
    }
}
