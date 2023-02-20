use super::model::FullStateResponse;
use crate::client::{error::OctoClientError, OctoClient};
use error_stack::Result;

/// # Printer operations
impl OctoClient {
    /// Retrieves the current state of the printer. 
    /// # Parameters
    /// - `history` set this to `true` if you want to retrieve historical temperature data
    /// - `limit` set this to [`Some<usize>`] to limit The number of history entries to return
    /// - Note that this endpoint also supports the `exclude` query parameter, but for my own sanity, I've left this for later
    /// 
    /// Requires the `STATUS` permission
    pub async  fn get_printer_state(
        &self,
        history: bool,
        limit: Option<usize>,
    ) -> Result<FullStateResponse, OctoClientError> {
        let url = "/api/printer";

        let mut request_builder = self.get(url)?;

        if history {
            request_builder = request_builder.query(&["history", true])
        }

        if let Some(limit_number) = limit {
            request_builder = request_builder.query(&["limit", limit_number])
        };

        let request = request_builder.build();

        let raw = self.execute(request).await?;

        let printer_state = self.parse::<FullStateResponse>(raw).await;

        printer_state
    }
}
