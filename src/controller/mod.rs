mod app_state;
mod integration_storage;
mod handler;
use crate::{
    entity::{config::Config, errors::SovesError},
    rpc_factory::RPCFactory,
};
use app_state::AppState;
use axum::{body::Body, extract::Request, http::Response, routing::get, Router};
use handler::decode_tx_handler;
use std::{sync::Arc, time::Duration};
use tower_http::{
    classify::ServerErrorsFailureClass,
    trace::TraceLayer,
};
use tracing::Span;
use uuid::Uuid;

// let app = Router::new()
//     .route("/", get(handler))
//     .layer(layer_one)
//     .layer(layer_two)
//     .layer(layer_three);
//
//          requests
//             |
//             v
// +----- layer_three -----+
// | +---- layer_two ----+ |
// | | +-- layer_one --+ | |
// | | |               | | |
// | | |    handler    | | |
// | | |               | | |
// | | +-- layer_one --+ | |
// | +---- layer_two ----+ |
// +----- layer_three -----+
//              |
//              v
//          responses
pub fn setup_router(config: Config) -> Result<Router, SovesError> {
    // Create the RPCFactory

    let tracing_layer = TraceLayer::new_for_http()
    .make_span_with(|_request: &Request<Body>| {
        let request_id = Uuid::new_v4().to_string();
        tracing::info_span!("", request_id = %request_id)
    })
    .on_request(|request: &Request<Body>, _span: &Span| {
        tracing::info!(method = %request.method(), path = %request.uri().path(), "request received")
    })
    .on_response(
        |response: &Response<Body>, latency: Duration, _span: &Span| {
            tracing::info!(status = %response.status(), latency = ?latency, "response sent")
        },
    )
    .on_failure(
        |error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
            tracing::error!(error = ?error, "request failed")
        },
    );

    let rpc_factory: Arc<RPCFactory> = Arc::new(RPCFactory::new(&config.chain_to_rpcs)?);
    let app_state: Arc<AppState> = Arc::new(AppState::new(&config, rpc_factory)?);
    let integration_storage = integration_storage::IntegrationStorage::new(&config);

    let app = Router::new()
        .route("/:chain_id/:tx_hash", get(decode_tx_handler))
        .with_state(Arc::clone(&app_state))
        .layer(tracing_layer);

    Ok(app)
}
