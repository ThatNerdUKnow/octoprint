use reqwest::{Request, RequestBuilder};

use super::{error::OctoClientError, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};

impl OctoClient {
    /// Wrapper around http get request, you only need to provide a relative path to the base url defined when you constructed OctoClient.
    /// This will automatically authenticate and build your request. please note that you still have to execute and await the request yourself
    pub(super) fn get(&self, path: &str) -> Result<Request, OctoClientError> {
        let url = self
            .append_path_to_base_url(path)
            .change_context(OctoClientError::BuildRequest)?;
        //let builder = self.client.get(url);
        let builder = self.add_auth(self.client.get(url));

        let request = builder
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;
        Ok(request)
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
