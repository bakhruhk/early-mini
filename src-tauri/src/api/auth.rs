/// Authentication and HTTP request helpers for the Early API v4.
///
/// The Early API uses a developer sign-in flow:
/// 1. POST /developer/sign-in with { apiKey, apiSecret }
/// 2. Receive a bearer token
/// 3. Use that token in the Authorization header for all subsequent requests
use serde_json::{json, Value};

const BASE_URL: &str = "https://api.early.app/api/v4";

/// Authenticate with the Early API using an API key and secret.
/// Returns the bearer token on success.
pub async fn sign_in(api_key: &str, api_secret: &str) -> Result<String, String> {
    let client = reqwest::Client::new();

    let resp = client
        .post(format!("{BASE_URL}/developer/sign-in"))
        .json(&json!({
            "apiKey": api_key,
            "apiSecret": api_secret
        }))
        .send()
        .await
        .map_err(|e| format!("Network error: {e}"))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("Authentication failed ({status}): {body}"));
    }

    let data: Value = resp.json().await.map_err(|e| format!("Parse error: {e}"))?;

    // The API returns { "token": "..." } on success
    data.get("token")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| "No token field in auth response".to_string())
}

/// Make an authenticated request to the Early API.
/// All API calls go through this function so we have a single place to handle auth headers.
pub async fn make_request(
    method: &str,
    path: &str,
    token: &str,
    body: Option<Value>,
) -> Result<Value, String> {
    let client = reqwest::Client::new();
    let url = format!("{BASE_URL}{path}");

    let mut req = match method {
        "GET" => client.get(&url),
        "POST" => client.post(&url),
        "PATCH" => client.patch(&url),
        "DELETE" => client.delete(&url),
        _ => return Err(format!("Unsupported HTTP method: {method}")),
    };

    req = req.header("Authorization", format!("Bearer {token}"));

    if let Some(body) = body {
        req = req.json(&body);
    }

    let resp = req.send().await.map_err(|e| format!("Network error: {e}"))?;

    // 204 No Content (e.g. from stop tracking when nothing is tracking)
    if resp.status().as_u16() == 204 {
        return Ok(json!({"success": true, "empty": true}));
    }

    // 401 Unauthorized — token expired, needs re-authentication
    if resp.status().as_u16() == 401 {
        return Err("UNAUTHORIZED".to_string());
    }

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("API error ({status}): {body}"));
    }

    resp.json().await.map_err(|e| format!("Parse error: {e}"))
}
