use std::env;
use dotenv::dotenv;

use octoprint::client::{AuthenticationMethod, OctoClient};

pub fn get_client() -> OctoClient {
    dotenv().ok();

    let url: String = env::var("OCTOPRINT_URL").unwrap();
    let key: String = env::var("OCTOPRINT_API_KEY").unwrap();

    let mut builder = OctoClient::new(url).unwrap();

    let creds: AuthenticationMethod = AuthenticationMethod::Bearer(key);
    builder = builder.use_credentials(creds);

    
    builder.build().unwrap()
}
