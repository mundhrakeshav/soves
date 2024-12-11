mod app_state;
mod handler;

use std::sync::{Arc, RwLock};

use app_state::AppState;
use axum::Router;
use handler::Handler;

use crate::{
    entity::{config::Config, errors::SovesError},
    rpc_factory::RPCFactory,
};

type SharedState = Arc<RwLock<AppState>>;


pub fn setup_router(config: Config) -> Result<Router, SovesError> {
    // Create the RPCFactory
    
    let rpc_factory = Arc::new(RPCFactory::new(&config.chain_to_rpcs)?);
    let app_state: Arc<AppState> = Arc::new(AppState::new(config, rpc_factory)?);
    let h: Handler = Handler::default();

    let app = Router::new()
        .route("/:chain_id/:tx_hash", h.make_decode_tx_handler()).with_state(Arc::clone(&app_state));

    Ok(app)
}
