use crate::client::{OctoClient, server::model::CurrentUser, error::OctoClientError};
use error_stack::Result;

// TODO /api/login
// TODO /api/logout
impl OctoClient{
    pub fn get_current_user(&self)->Result<CurrentUser,OctoClientError>{
        let request = self.get("/api/currentuser")?.build();
        todo!();
    }
}