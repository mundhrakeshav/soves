use crate::{entity::config::Config, integrations::openchain::Openchain};

pub struct IntegrationStorage {
    Openchain: Openchain
}

impl  IntegrationStorage {
    pub fn new(config: &Config) -> IntegrationStorage {
        IntegrationStorage {
            Openchain: Openchain::new(config.openchain_url.clone())
        }
    }
    
}