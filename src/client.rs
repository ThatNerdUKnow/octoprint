use error_stack::{IntoReport, Result};
use reqwest::Url;
use reqwest::{Client, IntoUrl};

mod builder;
pub mod error;
/// File operations and data model
pub mod file;
mod helpers;
/// Job operations and data model
pub mod job;
/// Printer operations and data model
pub mod printer;
/// Octoprint API operations and data model
pub mod server;

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
