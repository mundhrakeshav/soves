mod types;

use std::collections::HashMap;
use serde_derive::Deserialize;
use serde_derive::Serialize;
use crate::entity::errors::{IntegrationError, SovesError};


const pathLookup: &str = "/signature-database/v1/lookup";

pub struct Openchain {
    url: String,
}



impl Openchain {
    pub fn new(url: String) -> Openchain {
        Openchain { url }
    }

    pub async fn lookup(&self, selectors: Vec<String>) -> Result<HashMap<String, String>, IntegrationError> {
        let mut query_function = String::new();

        for selector in selectors {
            query_function = query_function + &selector + ",";
        }

        query_function = query_function.strip_suffix(",").unwrap().to_string();

        let mut query_params: HashMap<&str, String> = HashMap::new();
        query_params.insert("function", query_function);
        query_params.insert("filter", "true".to_string());

        let client = reqwest::Client::new();
        let url = format!("{}{}", self.url, pathLookup);

        let response = client.get(&url).query(&query_params).send().await.map_err(|e| {
            IntegrationError::OpenchainError(format!(
                "Error sending request: {}, status: {:?}",
                e.to_string(),
                e.status() // Fetch the status if available
            ))
        })?;

        // Check if the response status is not successful
        if !response.status().is_success() {
            let status_code = response.status();
            let body = response.text().await.unwrap_or_else(|_| "Failed to read error body".to_string());
            return Err(IntegrationError::OpenchainError(format!(
                "Request failed with status: {}, body: {}",
                status_code, body
            )));
        }

        // Parse the response into the expected type
        let response_body = response.json::<types::OpenChainResponse>().await.map_err(|e| {
            IntegrationError::OpenchainError(format!("Error parsing response: {}", e.to_string()))
        })?;

        let mut result: HashMap<String, String> = HashMap::new();
        for (selector, function) in response_body.result.function {
            if let Some(functions) = function {
                for f in functions {
                    result.insert(selector.clone(), f.name);
                }
            }
        }
        
        Ok(result)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_lookup() {
        let openchain = Openchain::new("https://api.openchain.xyz".to_string());
        let selectors = vec![
            "0x0902f1ac".to_string(),
            "0x09c182c3".to_string(),
            "0xd0e30db0".to_string(),
            "0xa9059cbb".to_string(),
            "0x70a08231".to_string(),
            "0x022c0d9f".to_string(),
        ];

        let result = openchain.lookup(selectors).await;
        println!("{:?}", result);
    }
}
