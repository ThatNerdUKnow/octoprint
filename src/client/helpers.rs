use std::{any::type_name, borrow::Cow};

use reqwest::{Request, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde_json::Value;

use super::{error::OctoClientError, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};

impl OctoClient {
    /// Wrapper around http get request.
    /// This will automatically authenticate your request.
    /// Please note that you sill have to build, execute and await the request yourself
    ///
    ///  # Arguments
    ///
    /// * `path` - A relative HTTP path
    pub(super) fn get(&self, path: &str) -> Result<RequestBuilder, OctoClientError> {
        let url = self
            .append_path_to_base_url(path)
            .change_context(OctoClientError::BuildRequest)
            .attach_printable_lazy(|| self.base_url.to_owned())
            .attach_printable_lazy(|| path.to_owned())?;
        //let builder = self.client.get(url);
        let builder = self.add_auth(self.client.get(url));

        Ok(builder)
    }

    /// Wrapper around http post request.
    /// This will automatically authenticate your request.
    /// Please note that you still have to build, execute and await the
    /// request yourself
    ///
    /// # Arguments
    ///
    /// * `path` - A relative HTTP path
    /// * `payload` - A serialized body for this post request
    pub(super) fn post(
        &self,
        path: &str,
        payload: &str,
    ) -> Result<RequestBuilder, OctoClientError> {
        let url = self
            .append_path_to_base_url(path)
            .change_context(OctoClientError::BuildRequest)
            .attach_printable_lazy(|| self.base_url.to_owned())
            .attach_printable_lazy(|| path.to_owned())?;

        let mut builder = self.client.post(url);
        builder = self.add_auth(builder);
        builder = builder.body(payload.to_owned());

        Ok(builder)
    }

    /// Execute a request and wrap it in an [`OctoClientError`] context in the event of an error
    pub(super) async fn execute(&self, request: Request) -> Result<Response, OctoClientError> {
        Ok(self
            .client
            .execute(request)
            .await
            .into_report()
            .change_context(OctoClientError::Execute)?)
    }

    /// Parse a response and wrap it in an [`OctoClientError`] in the event of an error
    pub(super) async fn parse<T: DeserializeOwned>(
        &self,
        raw: Response,
    ) -> Result<T, OctoClientError> {
        let body = raw
            .text()
            .await
            .into_report()
            .change_context(OctoClientError::Parse)?;

        let body_pretty = {
            let val = serde_json::from_str::<Value>(&body)
                .and_then(|value| serde_json::to_string_pretty(&value));
            val
        }
        .into_report()
        .change_context(OctoClientError::Parse)?;

        serde_json::from_str::<T>(&body_pretty)
            .into_report()
            .change_context(OctoClientError::Parse)
            .attach_printable_lazy(|| {
                let type_name = type_name::<T>();
                format!("Attempted to parse body as {type_name}")
            })
            .attach_printable_lazy(|| "Raw body")
            .attach_printable_lazy(|| body_pretty)
    }
    /// Add proper authentication headers depending on authentication method provided to [`crate::client::OctoClientBuilder::use_credentials`]
    fn add_auth(&self, request: RequestBuilder) -> RequestBuilder {
        match &self.auth_credentials {
            super::AuthenticationMethod::Bearer(token) => request.bearer_auth(token),
            super::AuthenticationMethod::Basic { username, password } => {
                request.basic_auth(username, Some(password))
            }
        }
    }
}
