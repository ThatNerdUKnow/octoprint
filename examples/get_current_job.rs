use std::{env, error::Error};

use octoprint::client::{OctoClient, OctoClientBuilder};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Get environment variables
    let url: String = env::var("OCTOPRINT_URL")?;
    let key: String = env::var("OCTOPRINT_API_KEY")?;
    
    // Instantiate a client
    let octo = OctoClient::new(&url)?.use_api_key(&key)?.build()?;

    // execute a query against the octoprint api to get the current job
    let currentJob = octo.current_job().await?;
    println!("{currentJob:?}");
    Ok(())
}
