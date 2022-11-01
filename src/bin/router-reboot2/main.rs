mod utils;

use quick_xml::events::{BytesStart, BytesText, Event};
use quick_xml::writer::Writer;
use quick_xml::Error as XmlError;
use std::error::Error;
use std::io::Cursor;
use std::str;

use utils::*;

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

    Ok(())
}
