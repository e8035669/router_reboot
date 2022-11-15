use cookie_store::Cookie;
use ctr::cipher::{KeyIvInit, StreamCipher};
use hex;
use hmac::{Hmac, Mac};
use inout::block_padding::generic_array::typenum;
use inout::block_padding::Pkcs7;
use inout::InOutBufReserved;
use md5::{Digest, Md5};
use quick_xml::{de, se};
use rand::rngs::OsRng;
use rand::RngCore;
use reqwest::blocking::Client;
use reqwest::header;
use reqwest_cookie_store::CookieStoreMutex;
use sha2::Sha256;
use std::error::Error;
use std::sync::Arc;
use std::time;
use url::Url;

use crate::utils::body::{LoginEnvelop, ResponseEnvelop};
use crate::utils::soap::SoapEnvelope;
use crate::utils::{CellularSmsMessageEnvelop, CellularSmsMessageResponseEnvelop, SmsMessage};

type Aes256Ctr64BE = ctr::Ctr64BE<aes::Aes256>;
type HmacSha256 = Hmac<Sha256>;

fn aes_ctr_encrypt(message: &[u8], key: &[u8]) -> Result<String, Box<dyn Error>> {
    let mut iv = [0u8; 16];
    OsRng.fill_bytes(&mut iv);

    let mut buffer = message.to_vec();
    let msg_len = buffer.len();
    let new_len = (msg_len / 16 + 1) * 16;
    buffer.resize(new_len, 0);

    let inout = InOutBufReserved::from_mut_slice(buffer.as_mut_slice(), msg_len)?;
    let mut blocks = inout.into_padded_blocks::<Pkcs7, typenum::U16>()?;

    let mut cipher = Aes256Ctr64BE::new(key.into(), &iv.into());

    for block in blocks.get_blocks() {
        cipher.apply_keystream_inout(block.into_buf());
    }

    if let Some(block) = blocks.get_tail_block() {
        cipher.apply_keystream_inout(block.into_buf());
    }

    Ok(format!(
        "{} {}",
        hex::encode_upper(buffer),
        hex::encode_upper(iv)
    ))
}

pub struct DLinkRouter {
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
    api_url: String,
    private_key_str: String,
    private_key: Vec<u8>,
}

impl DLinkRouter {
    pub fn new(api_url: &str) -> Result<DLinkRouter, Box<dyn Error>> {
        let cookie_store = CookieStoreMutex::default();
        let cookie_store = Arc::new(cookie_store);

        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            header::HeaderValue::from_static("text/xml; charset=utf-8"),
        );

        let client = Client::builder()
            .cookie_provider(cookie_store.clone())
            .default_headers(headers)
            .build()?;
        let default_key = "withoutloginkey";

        Ok(DLinkRouter {
            client,
            cookie_store,
            api_url: api_url.to_string(),
            private_key_str: default_key.to_string(),
            private_key: Vec::new(),
        })
    }

    fn update_uid(&self, uid: &str) -> Result<(), Box<dyn Error>> {
        let url = Url::parse(&self.api_url.as_str())?;
        let cookie = Cookie::parse(format!("uid={}", uid), &url)?;
        let mut store = self.cookie_store.lock().unwrap();
        store.insert(cookie, &url)?;
        Ok(())
    }

    fn send_soap_action(&self, action_name: &str, body: &[u8]) -> Result<String, Box<dyn Error>> {
        let h = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_millis();
        let h = (h % 2000000000000u128).to_string();

        let auth_msg = format!("{}{}", h, action_name);
        // println!("Message: {}", auth_msg);
        let sign = HmacSha256::new_from_slice(&self.private_key.as_slice())?
            .chain_update(auth_msg.as_bytes())
            .finalize()
            .into_bytes()
            .to_vec();
        let e = hex::encode_upper(sign);
        let auth = format!("{} {}", e, h);
        // println!("auth: {}", auth);

        let md5sum = Md5::new().chain_update(body).finalize().to_vec();
        let api_content = aes_ctr_encrypt(
            hex::encode_upper(md5sum).as_bytes(),
            hex::decode(self.private_key_str.as_bytes())?.as_slice(),
        )?;

        let ret = self
            .client
            .post(&self.api_url)
            .body(body.to_owned())
            .header("API-ACTION", action_name)
            .header("API-AUTH", auth)
            .header("API-CONTENT", api_content)
            .send()?
            .error_for_status()?;
        // println!("Ret: {:?}", ret);
        let text = ret.text()?;
        // println!("text: {}", text);
        Ok(text)
    }

    pub fn login(&mut self, password: &str) -> Result<(), Box<dyn Error>> {
        let login_request = SoapEnvelope::new(LoginEnvelop::with_request());
        let ser1 = se::to_string(&login_request)?;

        let resp1 = self
            .client
            .post(&self.api_url)
            .body(ser1)
            .header("API-ACTION", "Login")
            .send()?
            .error_for_status()?;
        let env1: SoapEnvelope<ResponseEnvelop> = de::from_str(resp1.text()?.as_str())?;
        let resp1 = env1.body.login_reponse.ok_or("No LoginResponse")?;

        let passwd = format!(
            "{}{}",
            resp1.public_key.ok_or("Expect public_key")?.0,
            password
        );
        let key1 = passwd.as_bytes();
        let sign1 = HmacSha256::new_from_slice(key1)?
            .chain_update(
                resp1
                    .challenge
                    .as_ref()
                    .ok_or("Expect challenge")?
                    .0
                    .as_bytes(),
            )
            .finalize()
            .into_bytes()
            .to_vec();
        let t_msg = hex::encode_upper(sign1);

        self.private_key_str = t_msg.clone();
        let key2 = t_msg.as_bytes();
        self.private_key = key2.to_vec();
        let sign2 = HmacSha256::new_from_slice(key2)?
            .chain_update(
                resp1
                    .challenge
                    .as_ref()
                    .ok_or("Expect challenge")?
                    .0
                    .as_bytes(),
            )
            .finalize()
            .into_bytes()
            .to_vec();

        let l_msg = hex::encode_upper(sign2);

        let cookie = resp1.cookie.ok_or("Expect cookie")?.0;
        self.update_uid(&cookie.as_str())?;

        let login_action = SoapEnvelope::new(LoginEnvelop::with_login(&l_msg.as_str()));
        let ser2 = se::to_string(&login_action)?;
        let resp2 = self
            .client
            .post(&self.api_url)
            .body(ser2)
            .header("API-ACTION", "Login")
            .send()?
            .error_for_status()?;

        let env2: SoapEnvelope<ResponseEnvelop> = de::from_str(resp2.text()?.as_str())?;
        let resp2 = env2.body;
        let login_result = &resp2
            .login_reponse
            .as_ref()
            .ok_or("Expect LoginResponse")?
            .login_result
            .0;

        if login_result == "success" {
            Ok(())
        } else {
            Err(format!("Login not success: \n{:?}", &resp2).into())
        }
    }

    pub fn get_sms(&mut self) -> Result<SmsMessage, Box<dyn Error>> {
        let req = SoapEnvelope::new(CellularSmsMessageEnvelop::default());
        let req_ser = se::to_string(&req)?;
        // println!("{:?}", req_ser);

        let text = self.send_soap_action("GetCellularSmsMessage", req_ser.as_bytes())?;

        let env: SoapEnvelope<CellularSmsMessageResponseEnvelop> = de::from_str(text.as_str())?;
        let body = env.body.response;

        if body.result.0 == "OK" {
            Ok(body.sms_message.ok_or("Expect SmsMessage")?)
        } else {
            Err(Box::from(format!("Response is not OK: {:?}", body)))
        }
    }
}
