use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url_serde;
use url_serde::SerdeUrl as Url;

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    pub name: String,
    pub display: String,
    pub path: String,
    #[serde(rename = "type")]
    pub model_type: ModelType,
    #[serde(rename = "typePath")]
    pub type_path: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModelType {
    Model,
    MachineCode,
    Folder,
}

#[derive(Serialize, Deserialize)]
pub struct Folder {
    pub children: Vec<FileInfo>,
    pub size: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub hash: Option<String>,
    pub size: Option<usize>,
    pub date: Option<usize>,
    pub origin: FileOrigin,
    pub refs: Option<References>,
    pub gcodeAnalysis: Option<GCodeAnalysis>,
    pub prints: Option<PrintHistory>,
    pub statistics: Option<PrintStatistics>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FileOrigin {
    Local,
    SDCard,
}

#[derive(Serialize, Deserialize)]
pub struct References {
    pub resource: Url,
    pub download: Option<Url>,
    pub model: Option<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct PrintHistory {
    pub success: usize,
    pub failure: usize,
    pub last_date: usize,
    pub last_print_time: f64,
    pub last_success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PrintStatistics {
    pub averagePrintTime: HashMap<String, usize>,
    pub lastPrintTime: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct GCodeAnalysis {
    pub estimatedPrintTime: Option<f64>,
    pub filament: Option<Filament>,
}

#[derive(Serialize, Deserialize)]
pub struct Filament {
    pub length: f64,
    pub volume: f64,
}

pub struct Dimensions {
    pub depth: f64,
    pub height: f64,
    pub width: f64,
}
#[derive(Serialize, Deserialize)]
pub struct PrintingArea {
    pub maxX: f64,
    pub maxY: f64,
    pub maxZ: f64,
    pub minX: f64,
    pub minY: f64,
    pub minZ: f64,
}
