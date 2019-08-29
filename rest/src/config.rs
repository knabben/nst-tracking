#[derive(Debug)]
pub struct Config {
    pub validator_endpoint: String,
    pub database_endpoint: String,
    pub rest_api_endpoint: String,
}

impl Config {
    pub fn validator_endpoint(&self) -> &str {
        &self.validator_endpoint
    }

    pub fn rest_api_endpoint(&self) -> &str {
        &self.rest_api_endpoint
    }

    pub fn database_endpoint(&self) -> &str {
        &self.database_endpoint
    }
}
