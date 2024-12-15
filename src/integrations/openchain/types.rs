use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenChainResponse {
    #[serde(rename = "ok")]
    pub ok: bool,
    
    #[serde(rename = "result")]
    pub result: OpenChainResult,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenChainResult {
    #[serde(rename = "function")]
    pub function: HashMap<String, Option<Vec<Function>>>,}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Function {
    #[serde(rename = "name")]
    pub name: String,
    
    #[serde(rename = "filtered")]
    pub filtered: bool,
}
