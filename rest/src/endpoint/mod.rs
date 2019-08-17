#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub username: String,
    pub password: String,
}