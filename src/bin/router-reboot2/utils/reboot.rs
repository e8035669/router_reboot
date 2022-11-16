use serde::{Deserialize, Serialize};

use crate::utils::Empty;
use crate::utils::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct RebootEnvelop {
    pub reboot_message: Empty,
}

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
