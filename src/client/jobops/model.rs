use serde::{Deserialize, Serialize};

use crate::client::fileops::model::{FileInfo, Filament};

#[derive(Serialize, Deserialize)]
pub struct JobInfo {
    pub file: FileInfo,
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<f64>,
    #[serde(rename="lastPrintTime")]
    pub last_print_time: Option<f64>,
    pub filament: Option<Filament>
}
