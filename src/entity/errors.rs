use axum::{response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug)]

pub enum SovesError {
    EnvReadFailed(String),
    FailedToParse(String),
    ClientNotFound(u32),
    ProviderError(String),
}

impl IntoResponse for SovesError {
    fn into_response(self) -> axum::response::Response {
       
                Json(serde_json::json!({"error": "error happened"})).into_response()
        
    }
}