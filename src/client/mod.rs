use reqwest::header::{HeaderMap, HeaderValue, InvalidHeaderValue};
use reqwest::Url;
use reqwest::{Client, ClientBuilder, IntoUrl};

pub struct OctoClient {
    client: Client,
    baseURL: Url,
}

pub struct OctoClientBuilder {
    builder: ClientBuilder,
    headers: HeaderMap,
    baseURL: Url,
}

impl OctoClient {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        OctoClientBuilder::new(url)
    }
}

impl OctoClientBuilder {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        Ok(OctoClientBuilder {
            builder: ClientBuilder::new(),
            headers: HeaderMap::new(),
            baseURL: url.into_url()?,
        })
    }

    pub fn build(self) -> Result<OctoClient, reqwest::Error> {
        let client: Client = self.builder.build()?;
        Ok(OctoClient {
            client: client,
            baseURL: self.baseURL,
        })
    }

    pub fn use_api_key<'a, K: AsRef<&'a str>>(
        mut self,
        key: K,
    ) -> Result<OctoClientBuilder, InvalidHeaderValue> {
        let mut token: String = String::from("Bearer ");
        token += key.as_ref();
        let header: HeaderValue = HeaderValue::from_str(&token)?;
        self.headers.append("Authorization", header);
        Ok(self)
    }
}
