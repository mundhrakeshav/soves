use axum::{
    body::Body,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use tracing::error;

#[derive(Debug, Clone)]
pub enum SovesError {
    EnvReadFailed(String),
    FailedToParse(String),
    ClientNotFound(u32),
    ProviderError(String),
}

impl SovesError {
    pub fn into_response_error(self) -> APIError {
        error!("into_response_error: {:?}", self);
        match self {
            Self::ClientNotFound(chain_id) => APIError {
                message: format!("chain: {}, is not supported", chain_id),
                status_code: StatusCode::NOT_FOUND,
            },
            Self::EnvReadFailed(_) | Self::FailedToParse(_) | Self::ProviderError(_) => APIError {
                message: String::from("internal server error"),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct APIError {
    message: String,
    status_code: StatusCode,
}

impl IntoResponse for APIError {
    fn into_response(self) -> Response<Body> {
        let body = Json(json!({
            "error": self.message,
        }));

        (self.status_code, body).into_response()
    }
}
