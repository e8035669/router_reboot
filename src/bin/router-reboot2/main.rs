mod utils;

use dotenv_codegen::dotenv;
use hex;
use quick_xml::de::from_str;
use quick_xml::de::Deserializer;
use quick_xml::reader::Reader;
use quick_xml::se::to_string;
use quick_xml::se::Serializer;
use quick_xml::writer::Writer;
use quick_xml::Error as XmlError;
use reqwest::blocking::Client;
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::str;

use utils::*;

fn main() -> Result<(), Box<dyn Error>> {
    let dlink_loginpw = dotenv!("DLINK_PASSWORD");
    let client = Client::new();

    let login_enp = SoapEnvelope::new(LoginEnvelop {
        login: Some(Login::with_request()),
    });
    let mut buffer = Vec::new();
    let mut ser = Serializer::new(&mut buffer);
    login_enp.serialize(&mut ser)?;

    let msg2 = str::from_utf8(&buffer)?.to_owned();
    println!("Request:\n{}", msg2);

    let res = client
        .post("http://192.168.1.254/DHMAPI/")
        .body(buffer.clone())
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("API-ACTION", "Login")
        .send()?;

    println!("Res: {:?}", res);

    let recv_text = res.text()?;

    println!("Recv: {}", recv_text);

    let des: SoapEnvelope<ResponseEnvelop> = from_str(&recv_text.as_str())?;
    println!("{:?}", des);
    let resp = des.body.login_reponse.ok_or("No response")?;

    let passwd = format!("{}{}", resp.public_key.value, dlink_loginpw);
    let key = hmac::Key::new(hmac::HMAC_SHA256, passwd.as_bytes());
    let signed = hmac::sign(&key, resp.challenge.value.as_bytes());
    let t_msg = hex::encode_upper(signed.as_ref());
    let key2 = hmac::Key::new(hmac::HMAC_SHA256, t_msg.as_bytes());
    let signed2 = hmac::sign(&key2, resp.challenge.value.as_bytes());
    let l_msg = hex::encode_upper(signed2.as_ref());

    println!("t = {}", t_msg);
    println!("l = {}", l_msg);

    let cookie = resp.cookie.value;

    let login_enp2 = SoapEnvelope::new(LoginEnvelop {
        login: Some(Login::with_login(&l_msg)),
    });

    let login_env = to_string(&login_enp2)?;

    let resp2 = client
        .post("http://192.168.1.254/DHMAPI/")
        .body(login_env)
        .header("Cookie", format!("uid={}", cookie))
        .header("Content-Type", "text/xml; charset=utf-8")
        .header("API-ACTION", "Login")
        .send()?;

    println!("Resp2: {:?}", resp2.text()?);

    Ok(())
}
