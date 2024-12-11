use std::sync::Arc;

use crate::{
    entity::{config::Config, errors::SovesError},
    rpc_factory::RPCFactory,
    services::decoding_service::DecodingService,
};

pub struct AppState {
    pub decoding_service: DecodingService<RPCFactory>,
}

impl AppState {
    pub fn new(config: Config, rpc_factory: Arc<RPCFactory>) -> Result<Self, SovesError> {
        // Create the DecodingService with a reference to the RPCFactory
        let decoding_service = DecodingService::new(rpc_factory);

        // Return the ServiceStorage with both components
        Ok(Self { decoding_service })
    }
}
