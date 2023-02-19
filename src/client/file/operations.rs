use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};

use super::model::{FileInfo, FileOrigin};
use crate::client::{error::OctoClientError, OctoClient};

impl OctoClient {
    pub async fn get_files(
        &self,
        recursive: bool,
        bypass_cache: bool,
        location: Option<FileOrigin>
    ) -> Result<RetrieveResponse, OctoClientError> {
        let url = match location{
            Some(location) => format!("/api/files/{location}"),
            None => "/api/files".to_owned(),
        };

        let mut builder = self.get(&url)?;

        if recursive {
            builder = builder.query(&("recursive", true));
        }

        if bypass_cache {
            builder = builder.query(&("force", true));
        }

        let request = builder
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw_response = self
            .client
            .execute(request)
            .await
            .into_report()
            .change_context(OctoClientError::Request)?;

            let response = raw_response
            .json()
            .await
            .into_report()
            .change_context(OctoClientError::Parse)?;
        
        Ok(response)
    }
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveResponse {
    files: Vec<FileInfo>,
    free: Option<String>,
}