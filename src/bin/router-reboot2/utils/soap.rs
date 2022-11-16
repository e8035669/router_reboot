use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct Empty;

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct Value(pub String);

impl Value {
    pub fn new(value: &str) -> Self {
        Self {
            0: value.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
pub struct Val<T>(pub T);

impl<T> Val<T> {
    #[allow(dead_code)]
    pub fn new(value: T) -> Self {
        Self { 0: value }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Default, Clone)]
#[serde(rename = "soap:Envelope")]
pub struct SoapEnvelope<T> {
    #[serde(rename = "xmlns:xsi")]
    pub xsi: String,
    #[serde(rename = "xmlns:xsd")]
    pub xsd: String,
    #[serde(rename = "xmlns:soap")]
    pub soap: String,
    #[serde(rename(serialize = "soap:Body", deserialize = "Body"))] // issue: #347
    pub body: T,
}

impl<T> SoapEnvelope<T> {
    pub fn new(body: T) -> SoapEnvelope<T> {
        SoapEnvelope {
            xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xsd: "http://www.w3.org/2001/XMLSchema".to_string(),
            soap: "http://schemas.xmlsoap.org/soap/envelope/".to_string(),
            body,
        }
    }
}
