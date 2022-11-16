use serde::{Deserialize, Serialize};

use crate::utils::Value;

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RebootResponse {
    #[serde(rename = "RebootResult")]
    pub result: Value,
}

#[derive(Debug, PartialEq, Deserialize, Serialize, Default, Clone)]
pub struct RebootReponseEnvelop {
    #[serde(rename = "RebootResponse")]
    pub response: RebootResponse,
}
