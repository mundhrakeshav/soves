use alloy::{
    primitives::TxHash,
    providers::RootProvider,
    rpc::types::trace::geth::CallFrame,
    transports::http::{Client, Http},
};

use super::errors::SovesError;

trait Parser {
    fn name() -> &'static str;
    fn parse();
}

pub trait ProviderFactory {
    fn client(&self, chain_id: u32) -> Option<&RootProvider<Http<Client>>>;
}
