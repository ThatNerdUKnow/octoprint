use reqwest::Request;
use thiserror::Error;

#[derive(Error)]
#[derive(Debug)]
pub enum BuilderError{
    #[error("There was a problem creating an OctoClientBuilder")]
    Construct,
    #[error("There was a problem building an OctoClient")]
    Build
}

#[derive(Error,Debug)]
pub enum OctoClientError{
    #[error("There was a problem creating an OctoClient")]
    Construct,
    #[error("There was a problem building a request")]
    BuildRequest,
    #[error("There was a problem executing a request")]
    Request
}