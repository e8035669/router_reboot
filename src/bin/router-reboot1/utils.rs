use lazy_static::lazy_static;
use regex::Regex;
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::error::Error;

lazy_static! {
    static ref TOKEN_REGEX: Regex = Regex::new(r"fvt_sess_token[ =]+'(.*)';").unwrap();
}

pub struct Router {
    client: Client,
    cgi_url: String,
    sess_token: Option<String>,
}

impl Router {
    pub fn new(cgi_url: &str) -> Result<Router, Box<dyn Error>> {
        Ok(Router {
            client: Client::builder().cookie_store(true).build()?,
            cgi_url: cgi_url.to_string(),
            sess_token: None,
        })
    }

    #[allow(dead_code)]
    pub fn get_action(&self, method: &str) -> reqwest::blocking::RequestBuilder {
        self.client
            .get(self.cgi_url.as_str())
            .query(&[("action", method)])
    }

    pub fn post_action(&self, method: &str) -> reqwest::blocking::RequestBuilder {
        self.client
            .post(self.cgi_url.as_str())
            .query(&[("action", method)])
    }

    pub fn login(&mut self, username: &str, password: &str) -> Result<(), Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("frashnum", "");
        params.insert("Frm_Logintoken", "189515");
        params.insert("login_username", username);
        params.insert("login_password", password);
        let res = self.post_action("LOGIN").form(&params).send()?;
        let res_text = res.text()?;

        let find_token = TOKEN_REGEX
            .captures(res_text.as_str())
            .ok_or("Token not found, maybe login failed")?;
        self.sess_token = Some(String::from(
            find_token.get(1).ok_or("Cannot get token")?.as_str(),
        ));

        Ok(())
    }

    pub fn reboot(&mut self) -> Result<(), Box<dyn Error>> {
        let mut params = HashMap::new();
        params.insert("Reboot", "1");
        params.insert("wwwctrl_paramlist", "reboot=1");
        params.insert(
            "sess_token",
            self.sess_token
                .as_ref()
                .ok_or("Need session token")?
                .as_str(),
        );
        let res = self.post_action("reboot").form(&params).send()?;
        let res_text = res.text()?;
        let find_token = TOKEN_REGEX
            .captures(res_text.as_str())
            .ok_or("Token not found, maybe login failed")?;
        self.sess_token = Some(String::from(
            find_token.get(1).ok_or("Cannot get token")?.as_str(),
        ));
        Ok(())
    }
}
