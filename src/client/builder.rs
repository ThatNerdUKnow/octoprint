use std::sync::Arc;

use super::{error::BuilderError, AuthenticationMethod, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};
use reqwest::{Client, ClientBuilder, IntoUrl, Url};

pub struct OctoClientBuilder {
    builder: ClientBuilder,
    base_url: Url,
    auth_credentials: Option<Arc<AuthenticationMethod>>,
}

impl OctoClientBuilder {
    pub fn new<U: IntoUrl>(url: U) -> Result<OctoClientBuilder, reqwest::Error> {
        let url_printable = url.as_str().to_owned();

        let url = url
            .into_url()
            .into_report()
            .attach_printable_lazy(|| format!("{} is not a valid base URL", url_printable))?;

        Ok(OctoClientBuilder {
            builder: ClientBuilder::new(),
            base_url: url,
            auth_credentials: None,
        })
    }

    pub fn build(self) -> Result<OctoClient, BuilderError> {
        let err = BuilderError::Build;
        let client: Client = self.builder.build().into_report().change_context(err)?;
        let credentials = self.auth_credentials.ok_or(BuilderError::Build)?;
        Ok(OctoClient {
            client,
            base_url: self.base_url,
            auth_credentials: credentials,
        })
    }

    pub fn use_credentials(mut self, creds: Arc<AuthenticationMethod>) -> Self {
        self.auth_credentials = Some(creds);
        self
    }
}
