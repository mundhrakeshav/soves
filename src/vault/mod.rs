use std::env::{self};
use vaultrs::kv2;

use crate::entity::{config::Config, errors::SovesError};
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
const KEY_VAULT_ADDR: &str = "VAULT_ADDR";
const KEY_VAULT_TOKEN: &str = "VAULT_TOKEN";

pub async fn get_vault_config(app_name: &str) -> Result<Config, SovesError> {
    let vault_addr = env::var(KEY_VAULT_ADDR).map_err(|e| {
        SovesError::EnvReadFailed(format!("Failed to read '{}': {}", KEY_VAULT_ADDR, e))
    })?;

    let vault_root = env::var(KEY_VAULT_TOKEN).map_err(|e| {
        SovesError::EnvReadFailed(format!("Failed to read '{}': {}", KEY_VAULT_TOKEN, e))
    })?;

    // Create a client
    let client = VaultClient::new(
        VaultClientSettingsBuilder::default()
            .address(vault_addr)
            .token(vault_root)
            .build()
            .unwrap(),
    )
    .unwrap();

    let secret: Config = kv2::read(&client, "apps", format!("{}/config", app_name).as_str())
        .await
        .unwrap();

    Ok(secret)
}
