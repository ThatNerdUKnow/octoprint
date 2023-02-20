use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};

use super::model::{FileInfo, FileOrigin};
use crate::client::{error::OctoClientError, OctoClient};

/// # File operations
impl OctoClient {
    /// Retrieve information regarding all files currently available and regarding the disk space still available locally in the system.
    /// The results are cached for performance reasons. By default, only returns files and folders in the root directory
    ///
    /// # Parameters
    /// - `recursive` return all files and folders recursively
    /// - `bypass_cache` bypass cached response from the server. Note that this does not affect files on the SD card
    /// - `location` optionally restrict files depending on which location they are stored.
    /// [`FileOrigin::SDCard`] To only return files on the SD card.
    /// [`FileOrigin::Local`] to only return files on local storage
    ///
    /// Requires the `FILES_LIST` permission
    pub async fn get_files(
        &self,
        recursive: bool,
        bypass_cache: bool,
        location: Option<FileOrigin>,
    ) -> Result<RetrieveResponse, OctoClientError> {
        let url = match location {
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

        let raw = self.execute(request).await?;

        let response = self.parse::<RetrieveResponse>(raw).await;

        response
    }

    /// Retrieves the selected file or folder's information.
    /// If the file is unknown, This operation will fail.
    /// If the targeted path is a folder, by default only its direct children will be returned
    ///
    /// # Parameters
    /// - `location` Which location to look for the requested file. [`FileOrigin::Local`] for local storage, [`FileOrigin::SDCard`] for files stored on the SD Card
    /// - `path` The name/path of the file for which to retrieve the information
    /// - `recursive` if set to true, return all files and folders recursively, otherwise only return items on the same level
    ///
    /// Requires the `FILES_LIST` permission
    pub async fn get_file(
        &self,
        location: FileOrigin,
        path: &str,
        recursive: bool,
    ) -> Result<FileInfo, OctoClientError> {
        let url = format!("/api/files/{location}/{path}");

        let mut request_builder = self.get(&url)?;

        if recursive == true {
            request_builder = request_builder.query(&[("recursive", true)])
        }

        let request = request_builder
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        let file_info = self.parse(raw).await;

        file_info
    }
}

#[derive(Serialize, Deserialize)]
pub struct RetrieveResponse {
    files: Vec<FileInfo>,
    free: Option<String>,
}
