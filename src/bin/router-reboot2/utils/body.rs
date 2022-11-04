use serde::{Deserialize, Serialize};

use crate::utils::Login;
use crate::utils::LoginResponse;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct LoginEnvelop {
    #[serde(rename = "Login")]
    pub login: Option<Login>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ResponseEnvelop {
    #[serde(rename = "LoginResponse")]
    pub login_reponse: Option<LoginResponse>,
}
