use crate::common::get_client;

#[tokio::test]
async fn get_job() {
    let octo = get_client();
    octo.current_job().await.unwrap();
}
