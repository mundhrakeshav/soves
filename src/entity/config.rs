use std::collections::HashMap;

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "chainID2RpcURLs")]
    pub chain_to_rpcs: HashMap<String, String>,
    #[serde(rename = "port")]
    pub host_port: String,
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "openchainURL")]
    pub openchain_url: String,
}
