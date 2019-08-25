pub mod agents;
pub mod products;
pub mod utils;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
  pub record_id: String,
  pub title: String,
  pub latitude: u32,
  pub longitude: u32,
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
