pub mod agents;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    token: String,
}
