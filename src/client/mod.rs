use reqwest::Url;
use reqwest::{Client, ClientBuilder, IntoUrl};
use error_stack::{Result, IntoReport, ResultExt};
use self::error::BuilderError;

pub mod error;
pub mod file;
mod helpers;
pub mod job;
pub mod printer;
mod builder;

pub use builder::OctoClientBuilder;

pub struct OctoClient {
    client: Client,
    base_url: Url,
    auth_credentials: AuthenticationMethod,
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

