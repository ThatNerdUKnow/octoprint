use crate::common::get_client;

#[tokio::test]
async fn current_user() {
    let octo = get_client();
    octo.get_current_user().await.unwrap();
}

#[tokio::test]
async fn server_version() {
    let octo = get_client();
    octo.server_version().await.unwrap();
}

#[tokio::test]
async fn server_info() {
    let octo = get_client();
    octo.server_information().await.unwrap();
}
