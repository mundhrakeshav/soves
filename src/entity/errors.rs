use serde::{Deserialize, Serialize};

#[derive(Debug)]

pub enum SovesError {
    EnvReadFailed(String),
    FailedToParse(String),
    ClientNotFound(u32),
    ProviderError(String),
}
