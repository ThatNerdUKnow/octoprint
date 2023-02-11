use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use url_serde;
use url_serde::SerdeUrl as Url;

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    /// The name of the file without path. E.G. "file.gco"
    /// for a file "file.gco" located anywhere in the file system.
    /// Currently this will always fit into ASCII.
    pub name: String,
    /// The name of the file without the path, this time potentially
    /// with non-ASCII unicode characters. E.G.
    /// “a turtle 🐢.gco” for a file “a_turtle_turtle.gco”
    /// located anywhere in the file system.
    pub display: String,
    /// The path to the file within the location. E.g.
    /// "folder/subfolder/file.gco" for a file
    /// "file.gco" located within "folder" and "subfolder"
    /// relative to the root of the location.
    /// Currently this will always fit nto ASCII
    pub path: PathBuf,
    /// Type of the file. `model` or `machinecode`. Or `folder`
    /// if it's a folder, in which case the children node will be populated
    #[serde(rename = "type")]
    pub model_type: ModelType,
    /// Path to type of file in extension tree. E.g.
    /// `["model","stl"]` for `.stl`
    /// files or `["machinecode","gcode"]` for 
    /// `.gcode` files. `["folder"]` for folders
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
    /// Contained children for entries
    /// of type `folder`. On non recursive listings only present on
    /// the first level sub folders!
    pub children: Option<Vec<FileInfo>>,
    /// The size of all files contained
    /// in the folder and its subfolders.
    /// Not present in non recursive listings!
    pub size: Option<usize>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub hash: Option<String>,
    pub size: Option<usize>,
    pub date: Option<usize>,
    pub origin: FileOrigin,
    pub refs: Option<References>,
    #[serde(rename = "gcodeAnalysis")]
    pub gcode_analysis: Option<GCodeAnalysis>,
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
    #[serde(rename = "averagePrintTime")]
    pub average_print_time: HashMap<String, usize>,
    #[serde(rename = "lastPrintTime")]
    pub last_print_time: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct GCodeAnalysis {
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<f64>,
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
#[serde(rename_all = "camelCase")]
pub struct PrintingArea {
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
}
