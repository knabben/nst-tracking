pub mod agents;
pub mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizeAgentRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    token: String,
}
