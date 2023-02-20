mod common;
use crate::common::get_client;
use lazy_static::lazy_static;
use octoprint::client::{file::model::FileOrigin, OctoClient};

lazy_static! {
    static ref OCTO: OctoClient = get_client();
}

#[tokio::test]
async fn get_files() {
    OCTO.get_files(false, false, None).await.unwrap();
}

#[tokio::test]
async fn get_files_recursive() {
    OCTO.get_files(true, false, None).await.unwrap();
}

#[tokio::test]
async fn get_files_nocache() {
    OCTO.get_files(false, true, None).await.unwrap();
}

#[tokio::test]
async fn get_files_local() {
    OCTO.get_files(false, false, Some(FileOrigin::Local))
        .await
        .unwrap();
}

#[tokio::test]
async fn get_files_sdcard() {
    OCTO.get_files(false, false, Some(FileOrigin::SDCard))
        .await
        .unwrap();
}
