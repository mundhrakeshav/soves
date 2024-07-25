use alloy::primitives::ChainId;

pub struct ChainConfig{
    chainID: ChainId,
}

impl ChainConfig {
    pub fn new(chainID: u64) -> Self {
        ChainConfig {
            chainID: ChainId::from(chainID),
        }
    }
}