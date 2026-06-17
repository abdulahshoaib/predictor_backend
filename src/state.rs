use dotenvy::dotenv;
use reqwest::Client;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub supabase_url: String,
    pub service_key: String,
    pub port: String,
}

impl AppState {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            client: Client::new(),
            supabase_url: env::var("SUPABASE_URL")
                .expect("SUPABASE_URL must be set in .env or the environment"),
            service_key: env::var("SUPABASE_SERVICE_ROLE_KEY")
                .expect("SUPABASE_SERVICE_ROLE_KEY must be set in .env or the environment"),
            port: env::var("PORT").unwrap(),
        }
    }
}
