use crate::storage_worker::BlockStorage;
use crate::{config::StorageConfig, error::StorageError};
use crypto::hash::Hash;
use logging::{debug, massa_trace};
use models::block::Block;
use std::collections::HashMap;

pub fn start_storage_controller(
    cfg: StorageConfig,
) -> Result<(StorageCommandSender, StorageManager), StorageError> {
    debug!("starting storage controller");
    massa_trace!("start", {});
    let db = BlockStorage::open(&cfg)?;
    Ok((StorageCommandSender(db.clone()), StorageManager(db)))
}

#[derive(Clone)]
pub struct StorageCommandSender(pub BlockStorage);

impl StorageCommandSender {
    pub async fn reset(&self) -> Result<(), StorageError> {
        let db = self.0.clone();
        tokio::task::spawn_blocking(move || db.reset()).await?
    }

    pub async fn add_block(&self, hash: Hash, block: Block) -> Result<(), StorageError> {
        let db = self.0.clone();
        tokio::task::spawn_blocking(move || db.add_block(hash, block)).await?
    }

    pub async fn get_block(&self, hash: Hash) -> Result<Option<Block>, StorageError> {
        let db = self.0.clone();
        tokio::task::spawn_blocking(move || db.get_block(hash)).await?
    }

    pub async fn contains(&self, hash: Hash) -> Result<bool, StorageError> {
        let db = self.0.clone();
        tokio::task::spawn_blocking(move || db.contains(hash)).await?
    }

    pub async fn get_slot_range(
        &self,
        start: (u64, u8),
        end: (u64, u8),
    ) -> Result<HashMap<Hash, Block>, StorageError> {
        let db = self.0.clone();
        tokio::task::spawn_blocking(move || db.get_slot_range(start, end)).await?
    }
}

pub struct StorageManager(pub BlockStorage);

impl StorageManager {
    pub async fn stop(self) -> Result<(), StorageError> {
        Ok(())
    }
}
