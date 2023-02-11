use std::error::Error;

use serde::{Serialize, Deserialize};

use crate::client::OctoClient;

use super::model::{JobInfo, ProgressInfo};

impl OctoClient {
    pub async fn current_job(&self) -> Result<JobInformationResponse, Box<dyn Error>> {
        let url = self.appendPathToBaseURL("/api/job")?;
        let request = self.client.get(url).build()?;
        let response = self.client.execute(request).await?.json::<JobInformationResponse>().await?;
        return Ok(response);        
    }
}

#[derive(Serialize,Deserialize)]
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
}
