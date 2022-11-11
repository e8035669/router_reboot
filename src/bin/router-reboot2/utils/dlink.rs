use cookie_store::Cookie;
use hex;
use quick_xml::{de, se};
use reqwest::blocking::Client;
use reqwest::header;
use reqwest_cookie_store::CookieStoreMutex;
use ring::hmac;
use std::error::Error;
use std::sync::Arc;
use std::time;
use url::Url;

use crate::utils::body::{LoginEnvelop, ResponseEnvelop};
use crate::utils::soap::SoapEnvelope;
use crate::utils::{CellularSmsMessageEnvelop, CellularSmsMessageResponseEnvelop, SmsMessage};

pub struct DLinkRouter {
    client: Client,
    cookie_store: Arc<CookieStoreMutex>,
    api_url: String,
    private_key_str: String,
    private_key: hmac::Key,
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
            private_key: hmac::Key::new(hmac::HMAC_SHA256, default_key.as_bytes()),
        })
    }

    fn update_uid(&self, uid: &str) -> Result<(), Box<dyn Error>> {
        let url = Url::parse(&self.api_url.as_str())?;
        let cookie = Cookie::parse(format!("uid={}", uid), &url)?;
        let mut store = self.cookie_store.lock().unwrap();
        store.insert(cookie, &url)?;
        Ok(())
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
        let key1 = hmac::Key::new(hmac::HMAC_SHA256, passwd.as_bytes());
        let sign1 = hmac::sign(
            &key1,
            resp1
                .challenge
                .as_ref()
                .ok_or("Expect challenge")?
                .0
                .as_bytes(),
        );
        let t_msg = hex::encode_upper(sign1.as_ref());

        self.private_key_str = t_msg.clone();
        let key2 = hmac::Key::new(hmac::HMAC_SHA256, t_msg.as_bytes());
        self.private_key = key2.clone();
        let sign2 = hmac::sign(
            &key2,
            resp1
                .challenge
                .as_ref()
                .ok_or("Expect challenge")?
                .0
                .as_bytes(),
        );
        let l_msg = hex::encode_upper(sign2.as_ref());

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
        println!("{:?}", req_ser);

        let h = time::SystemTime::now()
            .duration_since(time::UNIX_EPOCH)?
            .as_millis();
        let h = (h % 2000000000000u128).to_string();

        let message = format!("{}{}", h, "GetCellularSmsMessage");
        println!("Message: {}", message);
        let sign = hmac::sign(&self.private_key, message.as_bytes());
        let e = hex::encode_upper(sign.as_ref());
        let auth = format!("{} {}", e, h);
        println!("auth: {}", auth);

        let ret = self
            .client
            .post(&self.api_url)
            .body(req_ser)
            .header("API-ACTION", "GetCellularSmsMessage")
            .header("API-AUTH", auth)
            .send()?
            .error_for_status()?;
        println!("Ret: {:?}", ret);
        let text = ret.text()?;
        println!("text: {}", text);
        let env: SoapEnvelope<CellularSmsMessageResponseEnvelop> =
            de::from_str(text.as_str())?;
        let body = env.body.response;

        if body.result.0 == "OK" {
            Ok(body.sms_message.ok_or("Expect SmsMessage")?)
        } else {
            Err(Box::from(format!("Response is not OK: {:?}", body)))
        }
    }
}
