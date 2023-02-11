use url_serde::SerdeUrl as Url;
use serde::{Deserialize, Serialize};
use url_serde;

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
    pub size: Option<f64>,
}

#[derive(Serialize, Deserialize)]
pub struct File {
    pub hash: Option<String>,
    pub size: Option<usize>,
    pub date: Option<usize>,
    pub origin: FileOrigin,
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