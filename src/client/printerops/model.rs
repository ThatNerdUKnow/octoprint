use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrinterState {
    text: String,
    flags: PrinterStateFlags,
}

#[derive(Serialize, Deserialize)]
pub struct PrinterStateFlags {
    pub operational: bool,
    pub paused: bool,
    pub printing: bool,
    pub pausing: bool,
    pub cancelling: bool,
    #[serde(rename = "sdReady")]
    pub sd_ready: bool,
    pub error: bool,
    pub ready: bool,
    #[serde(rename = "closedError")]
    pub closed_error: bool,
}

#[derive(Serialize,Deserialize)]
pub struct TemperatureData{
    pub actual: f64,
    pub target: f64,
    pub offset: Option<f64>
}

// TODO historic temperature data point

// TODO temperature offset

#[derive(Serialize,Deserialize)]
pub struct ResendStats{
    pub count: usize,
    pub transmitted: usize,
    pub ratio: usize
}