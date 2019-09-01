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
    pub price: i64,
    pub latitude: i64,
    pub longitude: i64,
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
