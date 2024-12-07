mod integration_storage;
mod service_storage;

use service_storage::ServiceStorage;

use crate::{controller::setup_router, entity::errors::SovesError, rpc_factory::RPCFactory, vault};

pub async fn app(app_name: &str) -> Result<(), SovesError> {
    let config = vault::get_vault_config(app_name).await?;

    // make address before as ss consumes config
    let addr = format!("{}:{}", config.host, config.host_port);

    // Create the RPCFactory
    let rpc_factory = RPCFactory::new(&config.chain_to_rpcs)?;

    let ss = ServiceStorage::new(config, &rpc_factory)?;

    let app = setup_router(ss.decoding_service);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
