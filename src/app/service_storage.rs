use crate::{
    entity::{config::Config, errors::SovesError},
    rpc_factory::RPCFactory,
    services::decoding_service::DecodingService,
};

pub struct ServiceStorage<'a> {
    pub decoding_service: DecodingService<'a>,
}

impl<'a> ServiceStorage<'a> {
    pub fn new(config: Config, rpc_factory: &'a RPCFactory) -> Result<Self, SovesError> {
        // Create the DecodingService with a reference to the RPCFactory
        let decoding_service = DecodingService::new(&rpc_factory);

        // Return the ServiceStorage with both components
        Ok(Self { decoding_service })
    }
}
