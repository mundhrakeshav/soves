mod chain_config;
mod simulator_task;
mod worker_node;

use alloy::primitives::ChainId;
use chain_config::ChainConfig;
use std::collections::HashMap;
use worker_node::WorkerNode;

pub struct AnvilPool {
    cfg: HashMap<ChainId, ChainConfig>,
    workers: WorkerNode,
}

impl AnvilPool {
    // pub fn new(chainIDs: &[u64], ) -> Self {
    //     AnvilPool {
    //         cfg: ChainConfig::new(chainID),
    //     }
    // }
    // pub fn spawn() {
    // }
}
