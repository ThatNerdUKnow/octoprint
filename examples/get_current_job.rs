use std::{env, error::Error};
use dotenv::dotenv;

use octoprint::client::{OctoClient, OctoClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let url: String = env::var("OCTOPRINT_URL")?;
    let key: String = env::var("OCTOPRINT_API_KEY")?;
    let octo = OctoClient::new(&url)?.use_api_key(&key)?.build()?;

    let currentJob = octo.current_job().await?;
    println!("{currentJob:?}");
    Ok(())
}
