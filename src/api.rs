use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct HealthResponse {
    message: String,
}

#[server(Health, "/api", "GetJson", "health")]
pub async fn health() -> Result<HealthResponse, ServerFnError> {
    Ok(HealthResponse {
        message: "healthy".to_string(),
    })
}
