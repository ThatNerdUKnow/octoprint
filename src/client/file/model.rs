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
    /// MD5 hash of the file. Only available for
    /// `local` files
    pub hash: Option<String>,
    /// The size of the file in bytes. Only available
    /// for `local` files or `sdcard` files if the printer
    /// supports file siszes for sd card files.
    pub size: Option<usize>,
    /// The timestamp when this file was uploaded. Only available for `local` files
    pub date: Option<usize>,
    /// The origin of the file. `local` when stored in OctoPrint's `uploads` folder,
    /// `sdcard` when stored on the printer's SD card (if available)
    pub origin: FileOrigin,
    /// References relevant to this file, left out in
    /// abridged version
    pub refs: Option<References>,
    /// Information from the analysis of the GCODE file.
    /// if available. Left out in abridged version
    #[serde(rename = "gcodeAnalysis")]
    pub gcode_analysis: Option<GCodeAnalysis>,
    /// Information about previous prints of the file.
    /// Left out if the file has never been printed
    pub prints: Option<PrintHistory>,
    /// Statistics about the file, based on the previous print times.
    /// Left out if the file has never been printed
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
    /// The resource that represents the file or folder
    /// (e.g. for issuing commands to or for deleting)
    pub resource: Url,
    /// THe download URL for the file. Never present for folders.
    pub download: Option<Url>,
    /// The model from which this file was generated (e.g. an STL, currently not used).
    /// Never present for folders
    pub model: Option<Url>,
}

#[derive(Serialize, Deserialize)]
pub struct PrintHistory {
    /// Number of successful prints
    pub success: usize,
    /// Number of failed prints
    pub failure: usize,
    /// Last date this file was printed
    pub last_date: usize,
    /// Last print time in seconds
    pub last_print_time: f64,
    /// Whether the last print was a success or not
    pub last_success: bool,
}

#[derive(Serialize, Deserialize)]
pub struct PrintStatistics {
    /// Object that maps printer profile names to the last print time of the file, in seconds
    #[serde(rename = "averagePrintTime")]
    pub average_print_time: HashMap<String, usize>,
    /// Object that maps printer profile names to the average print time of the file, in seconds
    #[serde(rename = "lastPrintTime")]
    pub last_print_time: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct GCodeAnalysis {
    /// The estimated print time of the file, in seconds
    #[serde(rename = "estimatedPrintTime")]
    pub estimated_print_time: Option<f64>,
    /// The esimated usage of filament
    pub filament: Option<Filament>,
    pub dimenstions: Option<Dimensions>,
    /// Information regarding the size of the printing area
    #[serde(rename = "printingArea")]
    pub printing_area: Option<PrintingArea>,
}

#[derive(Serialize, Deserialize)]
pub struct Filament {
    /// The length of the filament used, in mm
    pub length: f64,
    /// The volume of filament used, in cm³
    pub volume: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Dimensions {
    pub depth: f64,
    pub height: f64,
    pub width: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PrintingArea {
    /// The maximum X coordinate of the printed model, in mm
    pub max_x: f64,
    /// The maximum Y coordinate of the printed model, in mm
    pub max_y: f64,
    /// The maximum Z coordinate of the printed model, in mm
    pub max_z: f64,
    /// The minimum X coordinate of the printed model, in mm
    pub min_x: f64,
    /// The minimum Y coordinate of the printed model, in mm
    pub min_y: f64,
    /// The minimum Z coordinate of the printed model, in mm
    pub min_z: f64,
}
