use std::sync::Arc;

use alloy::{primitives::TxHash, rpc::types::trace::geth::CallFrame};
use axum::{
    extract::{Path, State}, response::IntoResponse, routing::{get, MethodRouter}, Json
};
use serde::{Deserialize, Serialize};


use crate::entity::errors::SovesError;

use super::app_state::AppState;

#[derive(Default)]
pub struct Handler {}

#[derive(Deserialize, Serialize)]
struct PathDecodeTx {
    tx_hash: TxHash,
    chain_id: u32,
}

impl Handler {
    pub fn make_decode_tx_handler(&self) -> MethodRouter<Arc<AppState>> {
        async fn handle(
            Path(params): Path<PathDecodeTx>,
            State(state): State<Arc<AppState>>,
        ) -> Result<Json<CallFrame>, SovesError> {
            // Call the decoding service and handle errors
            let trace = state.decoding_service.decode(params.tx_hash, params.chain_id).await?;
    
            // On success, wrap in Json and return
            Ok(Json(trace))
        }

        get(handle)
    }
}
