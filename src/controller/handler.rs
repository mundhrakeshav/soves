use std::collections::HashMap;

use alloy::primitives::TxHash;
use axum::{
    extract::Path,
    response::IntoResponse,
    routing::{get, MethodRouter},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::services::decoding_service::DecodingService;

pub struct Handler<'a>{
    // Reference and type both live for 'a
    pub decoding_service: DecodingService<'a>,
}
#[derive(Deserialize, Serialize)]
struct PathDecodeTx {
    tx_hash: TxHash,
    chain_id: u32,
}

impl<'a> Handler<'a> {
    pub fn new(decoding_service: DecodingService<'a>) -> Self {
        Handler { decoding_service }
    }

    pub fn make_decode_tx_handler(&self) -> MethodRouter {
        get( |Path(params): Path<PathDecodeTx>| async  {
            let d = self.decoding_service;
            let trace = self.decoding_service.decode(params.tx_hash, params.chain_id).await;

            match trace {
                Ok(trace) => Json(trace).into_response(),
                Err(e) => {
                    Json("Json(e).into_response()").into_response()
                }
            }
        })
    }
}
