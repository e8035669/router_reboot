use serde::{Serialize, Deserialize};

use crate::utils::Value;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct Login {
    #[serde(rename = "Action")]
    pub action: Value,
    #[serde(rename = "Username")]
    pub username: Value,
    #[serde(rename = "LoginPassword")]
    pub login_password: Value,
    #[serde(rename = "Captcha")]
    pub captcha: Value,
}

impl Login {
    pub fn with_request() -> Self {
        Self {
            action: Value::new("request"),
            username: Value::new("Admin"),
            ..Default::default()
        }
    }

    pub fn with_login(password: &str) -> Self {
        Self {
            action: Value::new("login"),
            username: Value::new("Admin"),
            login_password: Value::new(password),
            captcha: Value::default(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct LoginResponse {
    #[serde(rename = "LoginResult")]
    pub login_result: Value,
    #[serde(rename = "Challenge")]
    pub challenge: Option<Value>,
    #[serde(rename = "Cookie")]
    pub cookie: Option<Value>,
    #[serde(rename = "PublicKey")]
    pub public_key: Option<Value>,
    #[serde(rename = "BackOff")]
    pub back_off: Value,
    #[serde(rename = "Crypto")]
    pub crypto: Option<Value>,
    #[serde(rename = "LockRemaining")]
    pub lock_remaining: Value,
    #[serde(rename = "BackOffRemaining")]
    pub back_off_remaining: Value,
}




