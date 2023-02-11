use serde::{Deserialize, Serialize, de::DeserializeOwned};

use crate::client::file::model::{FileInfo, Filament};

#[derive(Serialize, Deserialize)]
pub struct JobInfo {
    /// The file that is the target of the current print job
    pub file: FileInfo,
    /// The estimated print time for the file, in seconds
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<f64>,
    /// The print time of the last print of the file, in seconds
    #[serde(rename="lastPrintTime")]
    pub last_print_time: Option<f64>,
    /// Information regarding the estimated filament usage of the print job
    pub filament: Option<Filament>
}

#[derive(Serialize,Deserialize)]
pub struct ProgressInfo{
    /// Percentage of the completion of the current print job
    pub completion: f64,
    /// Current position in the file being printed in bytes from the beginning
    pub filepos: usize,
    /// Time already spent printing in seconds
    #[serde(rename="printTime")]
    pub print_time: usize,
    /// Estimate of time left to print in seconds
    #[serde(rename="printTimeLeft")]
    pub print_time_left: usize,
    /// Origin of the current time left estimate
    #[serde(rename="printTImeLeftOrigin")]
    pub print_time_left_origin: PrintTImeLeftOrigin
}

#[derive(Serialize,Deserialize)]
pub enum PrintTImeLeftOrigin{
    /// Based on a linear approximation of the progress in the file in bytes vs time
    #[serde(rename="linear")]
    Linear,
    /// Based on an analysis of the file
    #[serde(rename="analysis")]
    Analysis,
    /// calculated estimate after stabilization of linear estimation
    #[serde(rename="estimate")]
    Estimate,
    /// Based on the average total from past prints of the same model against the same printer profile
    #[serde(rename="average")]
    Average,
    /// Mixture of `estimate` and `analysis`
    #[serde(rename="mixed-analysis")]
    MixedAnalysis,
    /// Mixture of `estimate` and `average`
    #[serde(rename="mixed-average")]
    MixedAverage
}