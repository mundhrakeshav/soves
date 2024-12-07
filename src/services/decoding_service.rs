use crate::utils;
use crate::{entity::errors::SovesError, rpc_factory::RPCFactory};

use alloy::primitives::TxHash;
use alloy::rpc::types::trace::geth::CallFrame;

pub struct DecodingService<'a> {
    rpc_factory: &'a RPCFactory,
}

impl<'a> DecodingService<'a> {
    pub fn new(rpc_factory: &'a RPCFactory) -> Self {
        Self { rpc_factory }
    }

    pub async fn decode(&self, tx_hash: TxHash, chain_id: u32) -> Result<CallFrame, SovesError> {
        let trace = utils::trace::get_trace_for_hash(self.rpc_factory, tx_hash, chain_id).await?;
        Ok(trace)
    }
}
