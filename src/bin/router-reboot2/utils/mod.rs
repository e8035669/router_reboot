use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::reader::Reader;
use quick_xml::writer::Writer;
use quick_xml::Error as XmlError;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

mod soap;
mod body;
mod loginbody;
pub use soap::*;
pub use body::*;
pub use loginbody::*;

pub struct DLinkRouter {
    client: Client,
    api_url: String,
}

impl DLinkRouter {
    pub fn new(api_url: &str) -> Result<DLinkRouter, Box<dyn Error>> {
        Ok(DLinkRouter {
            client: Client::builder().cookie_store(true).build()?,
            api_url: api_url.to_string(),
        })
    }

    pub fn login(&mut self, password: &str) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
