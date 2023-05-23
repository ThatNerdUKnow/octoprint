use crate::common::get_client;
use octoprint::client::file::model::FileOrigin;

#[tokio::test]
async fn get_files() {
    let octo = get_client();
    octo.get_files(false, false, None).await.unwrap();
}

#[tokio::test]
async fn get_files_recursive() {
    let octo = get_client();
    octo.get_files(true, false, None).await.unwrap();
}

#[tokio::test]
async fn get_files_nocache() {
    let octo = get_client();
    octo.get_files(false, true, None).await.unwrap();
}

#[tokio::test]
async fn get_files_local() {
    let octo = get_client();
    octo.get_files(false, false, Some(FileOrigin::Local))
        .await
        .unwrap();
}

#[tokio::test]
async fn get_files_sdcard() {
    let octo = get_client();
    octo.get_files(false, false, Some(FileOrigin::SDCard))
        .await
        .unwrap();
}
