mod utils;

use quick_xml::de::Deserializer;
use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::se::Serializer;
use quick_xml::writer::Writer;
use quick_xml::Error as XmlError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Cursor;
use std::str;

use utils::*;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename = "soap:Envelope")]
struct SoapEnvelope<T> {
    #[serde(rename = "xmlns:xsi")]
    xsi: String,
    #[serde(rename = "xmlns:xsd")]
    xsd: String,
    #[serde(rename = "xmlns:soap")]
    soap: String,
    #[serde(rename = "soap:Body")]
    body: T,
}

impl<T> SoapEnvelope<T> {
    fn new(body: T) -> SoapEnvelope<T> {
        SoapEnvelope {
            xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xsd: "http://www.w3.org/2001/XMLSchema".to_string(),
            soap: "http://schemas.xmlsoap.org/soap/envelope/".to_string(),
            body,
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LoginEnvelop {
    #[serde(rename = "Login")]
    login: Option<Login>,

    command2: Vec<Command2>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Login {
    foo: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Command2 {
    bar: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut writer = Writer::new(Cursor::new(Vec::new()));

    writer
        .create_element("soap:Envelope")
        .with_attribute(("xmlns:xsi", "http://www.w3.org/2001/XMLSchema-instance"))
        .with_attribute(("xmlns:xsd", "http://www.w3.org/2001/XMLSchema"))
        .with_attribute(("xmlns:soap", "http://schemas.xmlsoap.org/soap/envelope/"))
        .write_inner_content(|writer| {
            writer
                .create_element("soap:Body")
                .write_inner_content(|writer| {
                    writer.create_element("Login").write_empty()?;
                    Ok(())
                })?;
            Ok(())
        })?;

    let msg = str::from_utf8(&writer.into_inner().into_inner())?.to_owned();

    println!("{}", msg);

    let login_enp = SoapEnvelope::new(LoginEnvelop {
        login: Some(Login {
            foo: "1234".to_owned(),
        }),
        command2: vec![Command2 { bar: 87 }, Command2 { bar: 100 }],
    });
    let mut buffer = Vec::new();
    let mut writer = Writer::new_with_indent(&mut buffer, b' ', 2);
    let mut ser = Serializer::with_root(writer, None);
    login_enp.serialize(&mut ser)?;

    let msg2 = str::from_utf8(&buffer)?.to_owned();
    println!("{}", msg2);

    Ok(())
}
