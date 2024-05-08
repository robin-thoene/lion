use leptos::{server, ServerFnError};

#[server(Health, "/api", "Url", "health")]
pub async fn health() -> Result<String, ServerFnError> {
    Ok("Healthy".to_string())
}

