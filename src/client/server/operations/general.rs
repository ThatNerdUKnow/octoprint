use crate::client::{error::OctoClientError, server::model::CurrentUser, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};

// TODO /api/login
// TODO /api/logout
impl OctoClient {
    /// Retrieves information about the current user
    pub async fn get_current_user(&self) -> Result<CurrentUser, OctoClientError> {
        let request = self
            .get("/api/currentuser")?
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        let current_user = self.parse::<CurrentUser>(raw).await;

        current_user
    }

    /// Retrieve information regarding server and API version.
    pub async fn server_version(&self) -> Result<VersionInfo, OctoClientError> {
        let request = self
            .get("/api/version")?
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        let version_info = self.parse::<VersionInfo>(raw).await;
        version_info
    }
}

#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    /// API Version
    api: String,
    /// Server version
    server: String,
    /// Server version plus the prefix `OctoPrint`
    /// (to determine that this is indeed a genuine OctoPrint instance)
    text: String,
}
