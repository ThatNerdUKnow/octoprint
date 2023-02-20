mod common;
use common::get_client;
use lazy_static::lazy_static;
use octoprint::client::OctoClient;

lazy_static! {
    static ref OCTO: OctoClient = get_client();
}

#[tokio::test]
async fn get_job() {
    OCTO.current_job().await.unwrap();
}
