use crate::client::{error::OctoClientError, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};
use serde::{Deserialize, Serialize};

use super::model::{JobInfo, ProgressInfo};

impl OctoClient {
    pub async fn current_job(&self) -> Result<JobInformationResponse, OctoClientError> {
        // Build http request
        let request = self.get("/api/job")?;

        // Execute request
        let raw_response = self
            .client
            .execute(request)
            .await
            .into_report()
            .change_context(OctoClientError::Request)?;

        // Parse response
        let response = raw_response
            .json::<JobInformationResponse>()
            .await
            .into_report()
            .change_context(OctoClientError::Request)?;
        return Ok(response);
    }
}
/*
#[derive(Serialize, Deserialize, Debug)]
pub struct JobInformationResponse {
    /// Information regarding the target of the current print job
    job: JobInfo,
    /// Information regarding the progress of the current print job
    progress: ProgressInfo,
    /// A textual representation of the current state of the job
    /// or connection. e.g. "Operational", "Printing", "Pausing",
    /// "Paused", "Cancelling", "Error", "Offline", "Offline after error",
    /// "Opening serial connection" ... - please note that this list is not exhaustive!
    state: String,
    /// Any error message for the job or connection. Only set if there has been an error
    error: Option<String>,
}*/

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JobInformationResponse {
    Online {
        /// Information regarding the target of the current print job
        job: JobInfo,
        /// Information regarding the progress of the current print job
        progress: ProgressInfo,
        /// A textual representation of the current state of the job
        /// or connection. e.g. "Operational", "Printing", "Pausing",
        /// "Paused", "Cancelling", "Error", "Offline", "Offline after error",
        /// "Opening serial connection" ... - please note that this list is not exhaustive!
        state: String,
    },
    Offline {
        /// Same as above state, but we'll assume if other props don't deserialize correctly, that the printer is offline
        state: String,
    },
    Error {
        /// Any error message for the job or connection. Only set if there has been an error
        error: Option<String>,
    },
    Unknown,
}
