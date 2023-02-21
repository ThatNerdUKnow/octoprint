use crate::common::get_client;
use lazy_static::lazy_static;
use octoprint::client::{printer::model::StateExclude, OctoClient};

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

#[tokio::test]
async fn printer_state_exclude_state() {
    let excluded = vec![StateExclude::State];
    let state = OCTO
        .get_printer_state(false, None, Some(excluded))
        .await
        .unwrap();

    assert!(state.state.is_none());
}

#[tokio::test]
async fn printer_state_exclude_temperature() {
    let excluded = vec![StateExclude::Temperature];
    let state = OCTO
        .get_printer_state(false, None, Some(excluded))
        .await
        .unwrap();

    assert!(state.temperature.is_none());
}

#[tokio::test]
async fn printer_state_exclude_sd() {
    let excluded = vec![StateExclude::SD];
    let state = OCTO
        .get_printer_state(false, None, Some(excluded))
        .await
        .unwrap();

    assert!(state.sd.is_none());
}