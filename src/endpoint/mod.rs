

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentRequest {
    pub username: String,
    pub password: String,
}