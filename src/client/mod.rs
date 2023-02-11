use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Url;
use reqwest::{Client, ClientBuilder, IntoUrl};

pub mod file;
pub mod job;
pub mod printer;
pub struct OctoClient {
    client: Client,
    base_url: Url,
}

pub struct OctoClientBuilder {
    builder: ClientBuilder,
    headers: HeaderMap,
    base_url: Url,
}

impl OctoClient {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        OctoClientBuilder::new(url)
    }

    fn appendPathToBaseURL<'a>(&self, path: &'a str) -> Result<Url, url::ParseError> {
        self.base_url.join(path)
    }
}

impl OctoClientBuilder {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        Ok(OctoClientBuilder {
            builder: ClientBuilder::new(),
            headers: HeaderMap::new(),
            base_url: url.into_url()?,
        })
    }

    pub fn build(self) -> Result<OctoClient, reqwest::Error> {
        let client: Client = self.builder.build()?;
        Ok(OctoClient {
            client: client,
            base_url: self.base_url,
        })
    }

    pub fn use_api_key<'a>(
        mut self,
        key: &'a str,
    ) -> Result<OctoClientBuilder, InvalidHeaderValue> {
        let mut token: String = String::from("Bearer ");
        token += key.as_ref();
        let header: HeaderValue = HeaderValue::from_str(&token)?;
        self.headers.append("Authorization", header);
        Ok(self)
    }
}
