use std::sync::Arc;

use crate::entity::errors::SovesError;
use crate::entity::traits::ClientProvider;
use crate::utils;

use alloy::primitives::TxHash;
use alloy::rpc::types::trace::geth::CallFrame;

pub struct DecodingService<P>
where
    P: ClientProvider,
{
    rpc_factory: Arc<P>,
}

impl<P> DecodingService<P>
where
    P: ClientProvider,
{
    pub fn new(rpc_factory: Arc<P>) -> Self
    where
        P: ClientProvider,
    {
        Self { rpc_factory }
    }

    pub async fn decode(&self, tx_hash: TxHash, chain_id: u32) -> Result<CallFrame, SovesError> {
        let provider = match self.rpc_factory.as_ref().get_client(chain_id) {
            None => return Err(SovesError::ClientNotFound(chain_id)),
            Some(provider) => provider,
        };

        let trace = utils::trace::get_trace_for_hash(provider, tx_hash).await?;
        let selectors = utils::trace::get_selectors_for_trace(&trace);
        println!("{:?}", selectors);
        Ok(trace)
    }
}
