use reqwest::RequestBuilder;

use super::{error::OctoClientError, OctoClient};
use error_stack::{Result, ResultExt};

impl OctoClient {
    /// Wrapper around http get request, you only need to provide a relative path to the base url defined when you constructed OctoClient.
    /// This will automatically authenticate and build your request. please note that you still have to execute and await the request yourself
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

    /// Add proper authentication headers depending on provided authentication method
    fn add_auth(&self, request: RequestBuilder) -> RequestBuilder {
        match &self.auth_credentials {
            super::AuthenticationMethod::Bearer(token) => request.bearer_auth(token),
            super::AuthenticationMethod::Basic { username, password } => {
                request.basic_auth(username, Some(password))
            }
        }
    }
}
