use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct Value {
    #[serde(rename = "$value")]
    pub value: String,
}

impl Value {
    pub fn empty() -> Self {
        Self {
            value: String::new(),
        }
    }

    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_owned(),
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
