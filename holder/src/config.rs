use std::env;

use dotenvy::dotenv;

pub struct AppConfig {
    pub api_url: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        dotenv().ok();

        let api_url = env::var("VERIFY_SIGNATURE_API_URL")
            .unwrap_or_else(|_| "http://localhost:3000/api/verify-signature".to_string());

        Self { api_url }
    }
}
