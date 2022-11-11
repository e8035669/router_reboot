use serde::{Deserialize, Serialize};

use crate::utils::Login;
use crate::utils::LoginResponse;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct LoginEnvelop {
    #[serde(rename = "Login")]
    pub login: Option<Login>,
}

impl LoginEnvelop {
    pub fn with_request() -> Self {
        Self {
            login: Some(Login::with_request()),
        }
    }

    pub fn with_login(password: &str) -> Self {
        Self {
            login: Some(Login::with_login(password)),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct ResponseEnvelop {
    #[serde(rename = "LoginResponse")]
    pub login_reponse: Option<LoginResponse>,
}
