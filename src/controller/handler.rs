use std::sync::Arc;

use alloy::{primitives::TxHash, rpc::types::trace::geth::CallFrame};
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::entity::errors::APIError;

use super::app_state::AppState;

#[derive(Deserialize, Serialize)]
pub struct PathDecodeTx {
    tx_hash: TxHash,
    chain_id: u32,
}

pub async fn decode_tx_handler(
    Path(params): Path<PathDecodeTx>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<CallFrame>, APIError> {
    // Call the decoding service and handle errors
    match state
        .decoding_service
        .decode(params.tx_hash, params.chain_id)
        .await
    {
        Ok(t) => Ok(Json(t)),
        Err(e) => Err(e.into_response_error()),
    }
}
