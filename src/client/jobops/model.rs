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

#[derive(Serialize,Deserialize)]
pub struct ProgressInfo{
    completion: f64,
    filepos: usize,
    #[serde(rename="printTime")]
    print_time: usize,
    #[serde(rename="printTimeLeft")]
    print_time_left: usize,
    #[serde(rename="printTImeLeftOrigin")]
    print_time_left_origin: PrintTImeLeftOrigin
}

#[derive(Serialize,Deserialize)]
pub enum PrintTImeLeftOrigin{
    #[serde(rename="linear")]
    Linear,
    #[serde(rename="analysis")]
    Analysis,
    #[serde(rename="estimate")]
    Estimate,
    #[serde(rename="average")]
    Average,
    #[serde(rename="mixed-analysis")]
    MixedAnalysis,
    #[serde(rename="mixed-average")]
    MixedAverage
}