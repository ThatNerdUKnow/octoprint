use reqwest::{RequestBuilder, Request, Response};

use super::{error::OctoClientError, OctoClient};
use error_stack::{Result, ResultExt, IntoReport};

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

    pub(super) async fn execute(&self,request:Request) -> Result<Response,OctoClientError>{
        Ok(self.client.execute(request).await.into_report().change_context(OctoClientError::Execute)?)
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
