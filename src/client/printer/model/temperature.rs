use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TemperatureState {
    /// Current temperature stats for tool n.
    /// Enumerations starts at 0 for the first tool. Not included
    /// if querying only bed state
    #[serde(flatten)]
    tool_temps: Option<HashMap<String,TemperatureData>>,
    /// Current temperature stats for the printer's heated bed.
    /// Not included if querying only tool state or if the currently selected printer
    /// profile does not have a heated bed
    bed: Option<TemperatureData>,
    /// Temperature history
    history: Vec<HistoricTemperatureDataPoint>,
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

#[derive(Serialize, Deserialize)]
pub struct HistoricTemperatureDataPoint {
    /// Timestamp of this data point
    time: usize,
    /// Temperature stats for tool n. Enumeration starts at
    /// 0 for the first tool. Not included if querying only bed state
    #[serde(flatten)]
    tool_temps: HashMap<String, TemperatureData>,
    /// Temperature stats for the printer's heated bed.
    /// Not included if querying only tool state
    bed: Option<TemperatureData>,
}

#[derive(Serialize, Deserialize)]
pub struct TemperatureOffset {
    /// Temperature offset for tool n. Enumeration starts at 0
    /// for the first tool
    #[serde(flatten)]
    tool_temps: HashMap<String, f64>,
    /// Temperature offset for the printer's heated bed
    bed: Option<f64>,
}
