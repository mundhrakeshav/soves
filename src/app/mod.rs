use tracing::info;

use crate::{controller::setup_router, entity::errors::SovesError, vault};

pub async fn app(app_name: &str) -> Result<(), SovesError> {
    let config = vault::get_vault_config(app_name).await?;

    // make address before as ss consumes config
    let addr = format!("{}:{}", config.host, config.host_port);

    let app = setup_router(config)?;

    let listener = tokio::net::TcpListener::bind(addr.clone()).await.unwrap();

    info!("listening on {}", addr);
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
