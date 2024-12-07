use axum::{routing::get, Router};
use handler::Handler;

use crate::services::decoding_service::{self, DecodingService};

pub mod handler;

pub fn setup_router<'a>(decoding_service: DecodingService<'a>) -> Router {
    let h: Handler<'_> = Handler::new(decoding_service);

    let app = Router::new().route("/:chain_id/:tx_hash", h.make_decode_tx_handler());

    app
}
