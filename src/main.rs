mod app;
mod controller;
mod entity;
mod rpc_factory;
mod services;
mod utils;
mod vault;

use app::app;
use entity::errors::SovesError;
use log::debug;
#[tokio::main]
async fn main() -> Result<(), SovesError> {
    tracing_subscriber::fmt::init();

    debug!("initializing app");
    app("soves").await
}
