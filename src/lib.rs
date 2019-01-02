#![deny(missing_debug_implementations, missing_copy_implementations,
    trivial_casts, trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces, unused_qualifications)]

use chrono::NaiveDate;
use log::debug;
use oauth2::{AuthType, Config as OAuth2Config};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, USER_AGENT};
use reqwest::Method;
use serde::{Deserialize, Serialize};

// TODO: how to re-export public names?
pub mod activities;
pub mod body;
pub mod date;
pub mod errors;
pub mod query;
pub mod serializers;
pub mod sleep;
pub mod user;

use crate::errors::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Token(oauth2::Token);

impl From<oauth2::Token> for Token {
    fn from(token: oauth2::Token) -> Self {
        Token(token)
    }
}

#[derive(Debug)]
pub struct FitbitClient {
    client: reqwest::Client,
    base_1: url::Url,
    base_1_2: url::Url,
}

impl FitbitClient {
    #[allow(clippy::new_ret_no_self)]
    pub fn new(token: &Token) -> Result<FitbitClient> {
        let mut headers = HeaderMap::new();

        let bearer = format!("Bearer {}", token.0.access_token);
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&bearer)
                .expect("Failed to form Bearer Auth header from the token"),
        );
        headers.insert(USER_AGENT, HeaderValue::from_static("fitbit-rs (0.1.0)"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| Error::Http(e))?;

        Ok(FitbitClient {
            client,
            base_1: url::Url::parse("https://api.fitbit.com/1/").unwrap(),
            base_1_2: url::Url::parse("https://api.fitbit.com/1.2/").unwrap(),
        })
    }

    pub fn user(&self) -> Result<String> {
        let url = self
            .base_1
            .join("user/-/profile.json")
            .map_err(|e| Error::Url(e))?;
        Ok(self
            .client
            .request(Method::GET, url)
            .send()
            .and_then(|mut r| r.text())?)
    }

    pub fn heart(&self, date: NaiveDate) -> Result<String> {
        let path = format!(
            "user/-/activities/heart/date/{}/1d.json",
            date.format("%Y-%m-%d")
        );
        let url = self.base_1.join(&path).map_err(|e| Error::Url(e))?;
        self.client
            .request(Method::GET, url)
            .send()
            .and_then(|mut r| r.text())
            .map_err(|e| Error::Http(e))
    }

    pub fn step(&self, date: NaiveDate) -> Result<String> {
        let path = format!(
            "user/-/activities/steps/date/{}/1d.json",
            date.format("%Y-%m-%d")
        );
        let url = self.base_1.join(&path).map_err(|e| Error::Url(e))?;
        Ok(self
            .client
            .request(Method::GET, url)
            .send()
            .and_then(|mut r| r.text())
            .map_err(|e| Error::Http(e))?)
    }

    fn do_get(&self, path: &str) -> Result<String> {
        let url = self.base_1.join(&path)?;
        debug!("GET - {:?}", url);
        Ok(self.client.get(url).send()?.text()?)
    }

    fn do_get_1_2(&self, path: &str) -> Result<String> {
        let url = self.base_1_2.join(&path)?;
        debug!("GET - {:?}", url);
        Ok(self.client.get(url).send()?.text()?)
    }
}

#[allow(missing_debug_implementations)]
pub struct FitbitAuth(OAuth2Config);

impl FitbitAuth {
    pub fn new(client_id: &str, client_secret: &str) -> FitbitAuth {
        let auth_url = "https://www.fitbit.com/oauth2/authorize";
        let token_url = "https://api.fitbit.com/oauth2/token";

        // Set up the config for the Github OAuth2 process.
        let mut config = OAuth2Config::new(client_id, client_secret, auth_url, token_url);

        // config = config.set_response_type(ResponseType::Token);
        config = config.set_auth_type(AuthType::BasicAuth);

        // This example is requesting access to the user's public repos and email.
        config = config.add_scope("activity");
        config = config.add_scope("heartrate");
        config = config.add_scope("profile");
        config = config.add_scope("weight");
        config = config.add_scope("sleep");

        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        // TODO configurable redirect?
        config = config.set_redirect_url("http://127.0.0.1:8080");

        FitbitAuth(config)
    }

    pub fn get_token(&self) -> Result<oauth2::Token> {
        let authorize_url = self.0.authorize_url();

        use std::process::Command;

        #[cfg(target_os = "linux")]
        {
            let mut cmd = Command::new("xdg-open");
            cmd.arg(authorize_url.as_str());
            let mut child = cmd.spawn()?;
            child.wait()?;
        }

        #[cfg(target_os = "macos")]
        {
            let mut cmd = Command::new("open");
            cmd.arg(authorize_url.as_str());
            let mut child = cmd.spawn()?;
            child.wait()?;
        }

        println!(
            "Your browser should open automatically. If not, open this URL in your browser:\n{}\n",
            authorize_url.to_string()
        );

        // FIXME avoid unwrap here
        let server =
            tiny_http::Server::http("127.0.0.1:8080").expect("could not start http listener");
        let request = server.recv()?;
        let url = request.url().to_string();
        let response = tiny_http::Response::from_string("Go back to your terminal :)");
        request.respond(response)?;

        let code = {
            // remove leading '/?'
            let mut parsed = url::form_urlencoded::parse(url[2..].as_bytes());

            let (_, value) = parsed
                .find(|pair| {
                    let &(ref key, _) = pair;
                    key == "code"
                })
                .ok_or(Error::OAuthCodeMissing)?;
            value.to_string()
        };

        // Exchange the code with a token.
        self.0.exchange_code(code).map_err(|e| Error::AuthToken(e))
    }

    pub fn exchange_refresh_token(&self, token: Token) -> Result<oauth2::Token> {
        match token.0.refresh_token {
            Some(t) => self
                .0
                .exchange_refresh_token(t)
                .map_err(|e| Error::AuthToken(e)),
            None => Err(Error::RefreshTokenMissing),
        }
    }
}
