use std::any::type_name;

use super::model::{FullStateResponse, StateExclude};
use crate::client::{error::OctoClientError, OctoClient};
use error_stack::{IntoReport, Result, ResultExt};
use reqwest::RequestBuilder;

/// # Printer operations
impl OctoClient {
    /// Retrieves the current state of the printer.
    /// # Parameters
    /// - `history` set this to `true` if you want to retrieve historical temperature data
    /// - `limit` set this to [`Some<usize>`] to limit The number of history entries to return
    /// - `exclude` a list of properties to exclude on the payload. Enumerated for convenience
    ///
    /// Requires the `STATUS` permission
    pub async fn get_printer_state(
        &self,
        history: bool,
        limit: Option<usize>,
        exclude: Option<Vec<StateExclude>>,
    ) -> Result<FullStateResponse, OctoClientError> {
        let url = "/api/printer";

        let mut request_builder:RequestBuilder = self.get(url)?;

        if history {
            request_builder = request_builder.query(&[("history", "true")])
        }

        if let Some(limit_number) = limit {
            request_builder = request_builder.query(&[("limit", &format!("{limit_number}"))])
        };

        if let Some(excluded_state) = exclude {
            let val = excluded_state
                .iter()
                .map(|e| format!("{e:?}"))
                .map(|e| e.to_lowercase())
                .collect::<Vec<String>>()
                .join(",");

            request_builder = request_builder.query(&[("exclude", &val)])
        }

        let request = request_builder
            .build()
            .into_report()
            .change_context(OctoClientError::BuildRequest)?;

        let raw = self.execute(request).await?;

        let printer_state = self.parse::<FullStateResponse>(raw).await;

        printer_state
    }
}
