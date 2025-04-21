use crate::security::crypto::CryptoEngine;
use crate::storage::block_store::BlockStore;
use dess::config::MainConfig;

#[derive(Debug, Clone)]
pub struct DaemonState {
    pub running: bool,
    pub initialized: bool,
}
#[derive(Debug, Clone)]
pub struct Daemon {
    pub config: MainConfig,
    pub crypto: CryptoEngine,
    pub block_store: BlockStore,
    pub state: DaemonState,
}

impl Daemon {
    pub fn new(config: MainConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let local_config = config.clone();
        let crypto = CryptoEngine::new_from_file(local_config.key_path.clone())?;
        let block_store = BlockStore::new(local_config.db_path, crypto.clone())?;
        Ok(Self {
            config,
            crypto,
            block_store,
            state: DaemonState {
                running: false,
                initialized: false,
            },
        })
    }
}