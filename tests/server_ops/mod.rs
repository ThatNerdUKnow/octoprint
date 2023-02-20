use crate::common::get_client;
use lazy_static::lazy_static;
use octoprint::client::OctoClient;

lazy_static! {
    static ref OCTO: OctoClient = get_client();
}

#[tokio::test]
async fn current_user() {
    OCTO.get_current_user().await.unwrap();
}

#[tokio::test]
async fn server_version() {
    OCTO.server_version().await.unwrap();
}

#[tokio::test]
async fn server_info() {
    OCTO.server_information().await.unwrap();
}
