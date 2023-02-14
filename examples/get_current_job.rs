use dotenv::dotenv;
use std::{env, error::Error};

use octoprint::client::{AuthenticationMethod, OctoClient};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // Get environment variables
    let url: String = env::var("OCTOPRINT_URL")?;
    let key: String = env::var("OCTOPRINT_API_KEY")?;

    let creds: AuthenticationMethod = AuthenticationMethod::Bearer(key);

    // Instantiate a client
    let octo = OctoClient::new(&url)?.use_credentials(creds).build()?;

    // execute a query against the octoprint api to get the current job
    let current_job = octo.current_job().await?;
    
    println!("{current_job:?}");
    Ok(())
}
