use error_stack::{IntoReport, Result, ResultExt};
use reqwest::{Client, ClientBuilder, IntoUrl, Url};

use super::{error::BuilderError, AuthenticationMethod, OctoClient};

pub struct OctoClientBuilder {
    builder: ClientBuilder,
    base_url: Url,
    auth_credentials: Option<AuthenticationMethod>,
}

impl OctoClientBuilder {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        Ok(OctoClientBuilder {
            builder: ClientBuilder::new(),
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