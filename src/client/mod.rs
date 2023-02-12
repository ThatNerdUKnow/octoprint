use reqwest::header::HeaderMap;
use reqwest::Url;
use reqwest::{Client, ClientBuilder, IntoUrl};
use error_stack::{Result, IntoReport, ResultExt};
use self::error::BuilderError;

pub mod error;
pub mod file;
mod helpers;
pub mod job;
pub mod printer;

pub struct OctoClient {
    client: Client,
    base_url: Url,
    auth_credentials: AuthenticationMethod,
}

pub struct OctoClientBuilder {
    builder: ClientBuilder,
    headers: HeaderMap,
    base_url: Url,
    auth_credentials: Option<AuthenticationMethod>,
}


pub enum AuthenticationMethod {
    Bearer(String),
    Basic { username: String, password: String },
}

impl OctoClient {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        OctoClientBuilder::new(url)
    }

    fn append_path_to_base_url<'a>(&self, path: &'a str) -> Result<Url, url::ParseError> {
        let url = self.base_url.join(path).into_report();
        url
    }
}

impl OctoClientBuilder {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        Ok(OctoClientBuilder {
            builder: ClientBuilder::new(),
            headers: HeaderMap::new(),
            base_url: url.into_url()?,
            auth_credentials: None,
        })
    }

    pub fn build(self) -> Result<OctoClient, BuilderError> {
        let err = BuilderError::Build;
        let client: Client = self.builder.build().into_report().change_context(err)?;
        let credentials = self.auth_credentials.ok_or(BuilderError::Build)?;
        Ok(OctoClient {
            client: client,
            base_url: self.base_url,
            auth_credentials: credentials,
        })
    }

    pub fn use_credentials(mut self, creds: AuthenticationMethod) -> Self {
        self.auth_credentials = Some(creds);
        self
    }
}
