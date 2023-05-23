use crate::common::get_client;
use octoprint::client::printer::model::StateExclude;

#[tokio::test]
async fn printer_state() {
    let octo = get_client();
    octo.get_printer_state(false, None, None).await.unwrap();
}

#[tokio::test]
/// TODO for some reason the executor drops before this future resolves. super weird considering it's virtually idendical to [`printer_state`]
async fn printer_state_history() {
    let octo = get_client();
    octo.get_printer_state(true, None, None).await.unwrap();
}

#[tokio::test]
async fn printer_state_history_limit() {
    let octo = get_client();
    octo.get_printer_state(true, Some(1), None).await.unwrap();
}

#[tokio::test]
async fn printer_state_limit() {
    let octo = get_client();
    octo.get_printer_state(false, Some(1), None).await.unwrap();
}

#[tokio::test]
async fn printer_state_exclude_state() {
    let octo = get_client();
    let excluded = vec![StateExclude::State];
    let state = octo
        .get_printer_state(false, None, Some(excluded))
        .await
        .unwrap();

    assert!(state.state.is_none());
}

#[tokio::test]
async fn printer_state_exclude_temperature() {
    let octo = get_client();
    let excluded = vec![StateExclude::Temperature];
    let state = octo
        .get_printer_state(false, None, Some(excluded))
        .await
        .unwrap();

    assert!(state.temperature.is_none());
}

#[tokio::test]
async fn printer_state_exclude_sd() {
    let octo = get_client();
    let excluded = vec![StateExclude::SD];
    let state = octo
        .get_printer_state(false, None, Some(excluded))
        .await
        .unwrap();

    assert!(state.sd.is_none());
}
