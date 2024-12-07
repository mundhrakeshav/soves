use std::collections::HashMap;

use alloy::{
    providers::{ProviderBuilder, RootProvider},
    transports::http::{reqwest::Url, Client, Http},
};

use crate::entity::{errors::SovesError, traits::ProviderFactory};

#[derive(Clone)]
pub struct RPCFactory {
    http_client: HashMap<u32, RootProvider<Http<Client>>>,
}

impl RPCFactory {
    pub fn new(chains_to_rpc: &HashMap<String, String>) -> Result<RPCFactory, SovesError> {
        let mut clients: HashMap<u32, RootProvider<Http<Client>>> = HashMap::new();

        for (chain, rpc_string) in chains_to_rpc {
            // Parse the chain ID from the string key
            let chain_id = chain.parse::<u32>().map_err(|e| {
                SovesError::FailedToParse(format!(
                    "Failed to parse chain string '{}' into u32: {}",
                    chain, e
                ))
            })?;

            // Parse the RPC URL from the string value
            let rpc_url = rpc_string.parse::<Url>().map_err(|e| {
                SovesError::FailedToParse(String::from(format!(
                    "failed to parse rpc string {} to url, err: {}",
                    rpc_string, e
                )))
            })?;

            // Build the provider using the parsed URL
            let provider = ProviderBuilder::new().on_http(rpc_url);

            // Insert the provider into the clients hashmap
            clients.insert(chain_id, provider);
        }

        // Return the RPCFactory with the constructed clients
        Ok(RPCFactory {
            http_client: clients,
        })
    }

    pub fn client(&self, chain_id: u32) -> Option<&RootProvider<Http<Client>>> {
        self.http_client.get(&chain_id)
    }
}

impl ProviderFactory for &RPCFactory {
    fn client(&self, chain_id: u32) -> Option<&RootProvider<Http<Client>>> {
        (*self).client(chain_id)
    }
}
