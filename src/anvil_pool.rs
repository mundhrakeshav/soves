mod chain_config;

use alloy::primitives::ChainId;
use chain_config::ChainConfig;

pub struct AnvilPool{
    cfg: ChainConfig,
}


impl AnvilPool {
    pub fn new(chainIDs: &[u64], ) -> Self {
        AnvilPool {
            cfg: ChainConfig::new(chainID),
        }
    }

    pub fn spawn() {
        
    }
}