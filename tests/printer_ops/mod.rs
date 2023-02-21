use crate::common::get_client;
use lazy_static::lazy_static;
use octoprint::client::OctoClient;

lazy_static! {
    static ref OCTO: OctoClient = get_client();
}

#[tokio::test]
async fn printer_state() {
    OCTO.get_printer_state(false, None, None).await.unwrap();
}

#[tokio::test]
async fn printer_state_history() {
    OCTO.get_printer_state(true, None, None).await.unwrap();
}

#[tokio::test]
async fn printer_state_history_limit() {
    OCTO.get_printer_state(true, Some(1), None).await.unwrap();
}

#[tokio::test]
async fn printer_state_limit() {
    OCTO.get_printer_state(false, Some(1), None).await.unwrap();
}
