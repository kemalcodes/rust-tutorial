// Rust Tutorial #17: HTTP Requests with Reqwest
// Demonstrates GET/POST requests, JSON parsing with serde,
// error handling, headers, timeouts, and the reqwest Client.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

// --- Data models for JSON ---

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    name: String,
    email: String,
    #[serde(default)]
    id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    #[serde(default)]
    id: Option<u64>,
    title: String,
    body: String,
    #[serde(rename = "userId")]
    user_id: u64,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(default)]
    data: Option<serde_json::Value>,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    headers: Option<HashMap<String, String>>,
    #[serde(default)]
    json: Option<serde_json::Value>,
    #[serde(default)]
    args: Option<HashMap<String, String>>,
}

// --- Basic GET request ---

async fn simple_get() -> Result<String, reqwest::Error> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    let body = response.text().await?;
    Ok(body)
}

// --- GET with JSON parsing ---

async fn get_json() -> Result<ApiResponse, reqwest::Error> {
    let response = reqwest::get("https://httpbin.org/get").await?;
    let api_response: ApiResponse = response.json().await?;
    Ok(api_response)
}

// --- GET with query parameters ---

async fn get_with_params() -> Result<ApiResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://httpbin.org/get")
        .query(&[("page", "1"), ("limit", "10")])
        .send()
        .await?;
    let api_response: ApiResponse = response.json().await?;
    Ok(api_response)
}

// --- POST with JSON body ---

async fn post_json() -> Result<ApiResponse, reqwest::Error> {
    let user = User {
        name: "Alex".to_string(),
        email: "alex@example.com".to_string(),
        id: None,
    };

    let client = reqwest::Client::new();
    let response = client
        .post("https://httpbin.org/post")
        .json(&user)
        .send()
        .await?;
    let api_response: ApiResponse = response.json().await?;
    Ok(api_response)
}

// --- Custom headers ---

async fn get_with_headers() -> Result<ApiResponse, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://httpbin.org/headers")
        .header("X-Custom-Header", "my-value")
        .header("Accept", "application/json")
        .send()
        .await?;
    let api_response: ApiResponse = response.json().await?;
    Ok(api_response)
}

// --- Timeout configuration ---

async fn get_with_timeout() -> Result<String, reqwest::Error> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let response = client
        .get("https://httpbin.org/get")
        .send()
        .await?;
    let body = response.text().await?;
    Ok(body)
}

// --- Checking response status ---

async fn check_status() -> Result<u16, reqwest::Error> {
    let response = reqwest::get("https://httpbin.org/status/200").await?;
    Ok(response.status().as_u16())
}

// --- Error handling with custom error type ---

#[derive(Debug)]
enum ApiError {
    Network(reqwest::Error),
    NotFound,
    ServerError(u16),
    ParseError(String),
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        ApiError::Network(err)
    }
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Network(e) => write!(f, "Network error: {}", e),
            ApiError::NotFound => write!(f, "Resource not found"),
            ApiError::ServerError(code) => write!(f, "Server error: {}", code),
            ApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

async fn fetch_with_error_handling(url: &str) -> Result<String, ApiError> {
    let response = reqwest::get(url).await?;

    match response.status().as_u16() {
        200..=299 => {
            let body = response.text().await?;
            Ok(body)
        }
        404 => Err(ApiError::NotFound),
        500..=599 => Err(ApiError::ServerError(response.status().as_u16())),
        _ => Err(ApiError::ServerError(response.status().as_u16())),
    }
}

// --- Reusable client with default settings ---

fn create_api_client() -> reqwest::Client {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .user_agent("rust-tutorial/1.0")
        .build()
        .expect("Failed to create HTTP client")
}

// --- Working with serde_json::Value for dynamic JSON ---

fn parse_dynamic_json(json_str: &str) -> Result<String, String> {
    let value: serde_json::Value =
        serde_json::from_str(json_str).map_err(|e| e.to_string())?;

    // Access fields dynamically
    let name = value
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");

    Ok(name.to_string())
}

// --- Serialize and deserialize examples ---

fn serialize_user() -> String {
    let user = User {
        name: "Sam".to_string(),
        email: "sam@example.com".to_string(),
        id: Some(1),
    };
    serde_json::to_string_pretty(&user).unwrap()
}

fn deserialize_user(json: &str) -> Result<User, serde_json::Error> {
    serde_json::from_str(json)
}

#[tokio::main]
async fn main() {
    println!("=== Simple GET ===");
    match simple_get().await {
        Ok(body) => println!("Response length: {} bytes", body.len()),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== GET JSON ===");
    match get_json().await {
        Ok(resp) => println!("URL: {:?}", resp.url),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== GET with Params ===");
    match get_with_params().await {
        Ok(resp) => println!("Args: {:?}", resp.args),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== POST JSON ===");
    match post_json().await {
        Ok(resp) => println!("Sent JSON: {:?}", resp.json),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Custom Headers ===");
    match get_with_headers().await {
        Ok(resp) => println!("Headers response: {:?}", resp.headers),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Timeout ===");
    match get_with_timeout().await {
        Ok(body) => println!("Got {} bytes with timeout", body.len()),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Status Check ===");
    match check_status().await {
        Ok(status) => println!("Status: {}", status),
        Err(e) => println!("Error: {}", e),
    }

    println!("\n=== Error Handling ===");
    match fetch_with_error_handling("https://httpbin.org/status/404").await {
        Ok(body) => println!("Body: {}", body),
        Err(e) => println!("Handled error: {}", e),
    }

    println!("\n=== Serde Examples ===");
    let json = serialize_user();
    println!("Serialized: {}", json);

    let user = deserialize_user(&json).unwrap();
    println!("Deserialized: {:?}", user);

    let client = create_api_client();
    println!("\nClient created with timeout and user-agent: {:?}", client);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serialize_user() {
        let json = serialize_user();
        assert!(json.contains("Sam"));
        assert!(json.contains("sam@example.com"));
    }

    #[test]
    fn test_deserialize_user() {
        let json = r#"{"name":"Alex","email":"alex@example.com","id":5}"#;
        let user = deserialize_user(json).unwrap();
        assert_eq!(user.name, "Alex");
        assert_eq!(user.email, "alex@example.com");
        assert_eq!(user.id, Some(5));
    }

    #[test]
    fn test_deserialize_user_without_id() {
        let json = r#"{"name":"Sam","email":"sam@example.com"}"#;
        let user = deserialize_user(json).unwrap();
        assert_eq!(user.name, "Sam");
        assert_eq!(user.id, None);
    }

    #[test]
    fn test_parse_dynamic_json() {
        let json = r#"{"name":"Alex","age":30}"#;
        let name = parse_dynamic_json(json).unwrap();
        assert_eq!(name, "Alex");
    }

    #[test]
    fn test_parse_dynamic_json_missing_name() {
        let json = r#"{"age":30}"#;
        let name = parse_dynamic_json(json).unwrap();
        assert_eq!(name, "unknown");
    }

    #[test]
    fn test_parse_dynamic_json_invalid() {
        let result = parse_dynamic_json("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_user_roundtrip() {
        let user = User {
            name: "Sam".to_string(),
            email: "sam@example.com".to_string(),
            id: Some(42),
        };
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user, parsed);
    }

    #[test]
    fn test_post_serialization() {
        let post = Post {
            id: Some(1),
            title: "Hello".to_string(),
            body: "World".to_string(),
            user_id: 5,
        };
        let json = serde_json::to_string(&post).unwrap();
        assert!(json.contains("\"userId\":5"));  // Tests #[serde(rename)]
    }

    #[test]
    fn test_create_api_client() {
        let client = create_api_client();
        // Client should be created without panicking
        drop(client);
    }

    #[test]
    fn test_api_error_display() {
        let err = ApiError::NotFound;
        assert_eq!(format!("{}", err), "Resource not found");

        let err = ApiError::ServerError(500);
        assert_eq!(format!("{}", err), "Server error: 500");

        let err = ApiError::ParseError("bad json".to_string());
        assert_eq!(format!("{}", err), "Parse error: bad json");
    }
}
