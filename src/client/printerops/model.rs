use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PrinterState {
    /// A textual representation of the current state of the printer
    /// e.g. "Operational" or "Printing"
    text: String,
    /// A few boolean printer state flags
    flags: PrinterStateFlags,
}

#[derive(Serialize, Deserialize)]
pub struct PrinterStateFlags {
    /// `true` if the printer is operational, `false` otherwise
    pub operational: bool,
    /// `true` if the printer is currently paused, `false` otherwise
    pub paused: bool,
    /// `true` if the printer is currently printing, `false` otherwise
    pub printing: bool,
    /// `true` if the printer is currently printing and is in the process of pausing, `false` otherwise
    pub pausing: bool,
    /// `true` if the printer is currently printing and in the process of pausing, `false` otherwise
    pub cancelling: bool,
    /// `true` if the printer's SD card is available and initialized, `false` otherwise. This is redundant information to the SD State
    #[serde(rename = "sdReady")]
    pub sd_ready: bool,
    /// `true` if an unrecoverable error occurred, `false` otherwise
    pub error: bool,
    /// `true` if the printer is operational and no data is currently being streamed to SD, so ready to recieve instructions
    pub ready: bool,
    /// `true` if the printer is disconnected (possibly due to an error), `false` otherwise
    #[serde(rename = "closedError")]
    pub closed_error: bool,
}

#[derive(Serialize, Deserialize)]
pub struct TemperatureData {
    /// Current temperature
    pub actual: f64,
    /// Target temperature, may be `None` if no target temperature is set
    pub target: Option<f64>,
    /// Currently configured temperature offset to apply, will be left out for historic temperature information
    pub offset: Option<f64>,
}

// TODO historic temperature data point

// TODO temperature offset

#[derive(Serialize, Deserialize)]
pub struct ResendStats {
    /// Number of resend requests recieved since connecting
    pub count: usize,
    /// Number of transmitted lines since connecting
    pub transmitted: usize,
    /// Percentage of resend requests vs transmitted lines. Value between 0 and 100
    pub ratio: usize,
}
