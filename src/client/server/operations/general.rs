use crate::client::{error::OctoClientError, server::model::CurrentUser, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};

// TODO /api/login
// TODO /api/logout
/// # Server operations
impl OctoClient {
    /// Retrieves information about the current user
    pub async fn get_current_user(&self) -> Result<CurrentUser, OctoClientError> {
        let request = self
            .get("/api/currentuser")?
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        

        self.parse::<CurrentUser>(raw).await
    }

    /// Retrieve information regarding server and API version.
    pub async fn server_version(&self) -> Result<VersionInfo, OctoClientError> {
        let request = self
            .get("/api/version")?
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        
        self.parse::<VersionInfo>(raw).await
    }

    /// Retrieve information regarding server status.
    pub async fn server_information(&self) -> Result<ServerInfo, OctoClientError> {
        let request = self
            .get("/api/server")?
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        

        self.parse::<ServerInfo>(raw).await
    }
}

#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    /// API Version
    pub api: String,
    /// Server version
    pub server: String,
    /// Server version plus the prefix `OctoPrint`
    /// (to determine that this is indeed a genuine OctoPrint instance)
    pub text: String,
}

/// Information regarding server status
#[derive(Serialize, Deserialize)]
pub struct ServerInfo {
    /// Contains the server version
    pub version: String,
    /// Indicates if the server is running in safe mode
    pub safemode: Option<SafeMode>,
}

#[derive(Serialize, Deserialize)]
pub enum SafeMode {
    #[serde(rename = "settings")]
    Settings,
    #[serde(rename = "incomplete_startup")]
    IncompleteStartup,
    #[serde(rename = "flag")]
    Flag,
    /// This should always be false, but I'll come back to it later if I can figure out how to get
    /// serde to do that
    NotSafeMode(bool),
}
