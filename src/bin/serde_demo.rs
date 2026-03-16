// Rust Tutorial #22: Serialization with Serde
// Demonstrates #[derive(Serialize, Deserialize)], serde attributes,
// custom serializers, JSON/TOML formats, and serde_json::Value.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ========================================================
// Basic Derive
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct User {
    name: String,
    email: String,
    age: u32,
}

fn basic_json_roundtrip() -> User {
    let user = User {
        name: "Alex".to_string(),
        email: "alex@example.com".to_string(),
        age: 25,
    };

    let json = serde_json::to_string(&user).unwrap();
    let parsed: User = serde_json::from_str(&json).unwrap();
    assert_eq!(user, parsed);
    parsed
}

// ========================================================
// Serde Attributes — rename
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct ApiResponse {
    #[serde(rename = "statusCode")]
    status_code: u16,

    #[serde(rename = "responseBody")]
    response_body: String,

    #[serde(rename = "isSuccess")]
    is_success: bool,
}

fn rename_example() -> String {
    let response = ApiResponse {
        status_code: 200,
        response_body: "OK".to_string(),
        is_success: true,
    };
    serde_json::to_string(&response).unwrap()
}

// ========================================================
// Serde Attributes — rename_all
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Config {
    database_url: String,
    max_connections: u32,
    enable_logging: bool,
}

fn rename_all_example() -> String {
    let config = Config {
        database_url: "sqlite:db.sqlite".to_string(),
        max_connections: 10,
        enable_logging: true,
    };
    serde_json::to_string(&config).unwrap()
}

// ========================================================
// Serde Attributes — skip, default, alias
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserProfile {
    name: String,

    #[serde(default)]
    bio: String,

    #[serde(default = "default_role")]
    role: String,

    #[serde(skip_serializing)]
    password_hash: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    avatar_url: Option<String>,

    #[serde(alias = "user_name", alias = "username")]
    display_name: String,
}

fn default_role() -> String {
    "user".to_string()
}

fn skip_and_default_example() -> (String, UserProfile) {
    let profile = UserProfile {
        name: "Alex".to_string(),
        bio: "Rust developer".to_string(),
        role: "admin".to_string(),
        password_hash: "secret_hash_123".to_string(),
        avatar_url: None,
        display_name: "alex_codes".to_string(),
    };

    let json = serde_json::to_string_pretty(&profile).unwrap();
    // password_hash is skipped, avatar_url is skipped (None)

    // Deserialize with defaults
    let minimal_json = r#"{
        "name": "Sam",
        "display_name": "sam_dev",
        "password_hash": ""
    }"#;
    let parsed: UserProfile = serde_json::from_str(minimal_json).unwrap();
    // bio defaults to "", role defaults to "user"

    (json, parsed)
}

// ========================================================
// Enum Serialization
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

fn enum_tagged_example() -> Vec<String> {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle {
            width: 10.0,
            height: 20.0,
        },
        Shape::Triangle {
            base: 8.0,
            height: 6.0,
        },
    ];

    shapes
        .iter()
        .map(|s| serde_json::to_string(s).unwrap())
        .collect()
}

// Externally tagged (default)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Message {
    Text(String),
    Number(i64),
    Pair(String, i64),
}

// Untagged — tries each variant until one works
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum FlexibleValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Boolean(bool),
}

// ========================================================
// serde_json::Value — Dynamic JSON
// ========================================================

fn dynamic_json_example() -> serde_json::Value {
    // Create JSON dynamically
    let value = serde_json::json!({
        "name": "Alex",
        "age": 25,
        "tags": ["rust", "programming"],
        "address": {
            "city": "Berlin",
            "country": "Germany"
        }
    });
    value
}

fn navigate_dynamic_json(value: &serde_json::Value) -> Vec<String> {
    let mut results = vec![];

    // Access string field
    if let Some(name) = value.get("name").and_then(|v| v.as_str()) {
        results.push(format!("name: {}", name));
    }

    // Access number field
    if let Some(age) = value.get("age").and_then(|v| v.as_i64()) {
        results.push(format!("age: {}", age));
    }

    // Access array
    if let Some(tags) = value.get("tags").and_then(|v| v.as_array()) {
        let tag_list: Vec<&str> = tags.iter().filter_map(|t| t.as_str()).collect();
        results.push(format!("tags: {}", tag_list.join(", ")));
    }

    // Access nested object
    if let Some(city) = value.get("address").and_then(|a| a.get("city")).and_then(|c| c.as_str()) {
        results.push(format!("city: {}", city));
    }

    results
}

// Modify dynamic JSON
fn modify_json(mut value: serde_json::Value) -> serde_json::Value {
    if let Some(obj) = value.as_object_mut() {
        obj.insert("updated".to_string(), serde_json::json!(true));
        obj.insert("version".to_string(), serde_json::json!(2));
    }
    value
}

// ========================================================
// Flatten — merge nested structs
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Pagination {
    page: u32,
    per_page: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct UserListRequest {
    query: String,
    #[serde(flatten)]
    pagination: Pagination,
}

fn flatten_example() -> String {
    let request = UserListRequest {
        query: "rust".to_string(),
        pagination: Pagination {
            page: 1,
            per_page: 20,
        },
    };
    // Produces: {"query":"rust","page":1,"per_page":20}
    // NOT: {"query":"rust","pagination":{"page":1,"per_page":20}}
    serde_json::to_string(&request).unwrap()
}

// ========================================================
// HashMap serialization
// ========================================================

fn hashmap_json() -> String {
    let mut scores: HashMap<String, i32> = HashMap::new();
    scores.insert("Alex".to_string(), 95);
    scores.insert("Sam".to_string(), 87);
    scores.insert("Jordan".to_string(), 92);
    serde_json::to_string(&scores).unwrap()
}

// ========================================================
// TOML format
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct AppConfig {
    title: String,
    debug: bool,
    database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

fn toml_example() -> (String, AppConfig) {
    let config = AppConfig {
        title: "My App".to_string(),
        debug: true,
        database: DatabaseConfig {
            url: "sqlite:app.db".to_string(),
            max_connections: 5,
        },
    };

    let toml_str = toml::to_string_pretty(&config).unwrap();
    let parsed: AppConfig = toml::from_str(&toml_str).unwrap();
    (toml_str, parsed)
}

fn parse_toml_string() -> AppConfig {
    let toml_str = r#"
        title = "My App"
        debug = false

        [database]
        url = "postgres://localhost/mydb"
        max_connections = 10
    "#;
    toml::from_str(toml_str).unwrap()
}

// ========================================================
// Converting between formats
// ========================================================

fn json_to_toml(json_str: &str) -> Result<String, String> {
    let value: AppConfig = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
    toml::to_string_pretty(&value).map_err(|e| e.to_string())
}

fn toml_to_json(toml_str: &str) -> Result<String, String> {
    let value: AppConfig = toml::from_str(toml_str).map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|e| e.to_string())
}

// ========================================================
// Vec and nested structures
// ========================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Team {
    name: String,
    members: Vec<User>,
    #[serde(default)]
    tags: Vec<String>,
}

fn nested_structures() -> String {
    let team = Team {
        name: "Rust Team".to_string(),
        members: vec![
            User {
                name: "Alex".to_string(),
                email: "alex@example.com".to_string(),
                age: 25,
            },
            User {
                name: "Sam".to_string(),
                email: "sam@example.com".to_string(),
                age: 30,
            },
        ],
        tags: vec!["rust".to_string(), "backend".to_string()],
    };
    serde_json::to_string_pretty(&team).unwrap()
}

fn main() {
    println!("=== Basic Roundtrip ===");
    let user = basic_json_roundtrip();
    println!("User: {:?}", user);

    println!("\n=== Rename ===");
    println!("{}", rename_example());

    println!("\n=== Rename All ===");
    println!("{}", rename_all_example());

    println!("\n=== Skip and Default ===");
    let (json, parsed) = skip_and_default_example();
    println!("JSON (no password, no avatar): {}", json);
    println!("Parsed with defaults: {:?}", parsed);

    println!("\n=== Enum Tagged ===");
    for s in enum_tagged_example() {
        println!("  {}", s);
    }

    println!("\n=== Dynamic JSON ===");
    let value = dynamic_json_example();
    for line in navigate_dynamic_json(&value) {
        println!("  {}", line);
    }

    println!("\n=== Flatten ===");
    println!("{}", flatten_example());

    println!("\n=== HashMap ===");
    println!("{}", hashmap_json());

    println!("\n=== TOML ===");
    let (toml_str, _config) = toml_example();
    println!("{}", toml_str);

    println!("\n=== Nested Structures ===");
    println!("{}", nested_structures());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_roundtrip() {
        let user = User {
            name: "Alex".to_string(),
            email: "alex@example.com".to_string(),
            age: 25,
        };
        let json = serde_json::to_string(&user).unwrap();
        let parsed: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user, parsed);
    }

    #[test]
    fn test_rename() {
        let json = rename_example();
        assert!(json.contains("statusCode"));
        assert!(json.contains("responseBody"));
        assert!(json.contains("isSuccess"));
        assert!(!json.contains("status_code"));
    }

    #[test]
    fn test_rename_all_camel_case() {
        let json = rename_all_example();
        assert!(json.contains("databaseUrl"));
        assert!(json.contains("maxConnections"));
        assert!(json.contains("enableLogging"));
    }

    #[test]
    fn test_skip_serializing() {
        let (json, _) = skip_and_default_example();
        assert!(!json.contains("password_hash"));
        assert!(!json.contains("secret_hash"));
        assert!(!json.contains("avatar_url")); // None is skipped
    }

    #[test]
    fn test_default_values() {
        let json = r#"{"name":"Sam","display_name":"sam","password_hash":""}"#;
        let profile: UserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.bio, ""); // default
        assert_eq!(profile.role, "user"); // custom default
    }

    #[test]
    fn test_alias() {
        // "username" alias
        let json = r#"{"name":"Alex","username":"alex_dev","password_hash":""}"#;
        let profile: UserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.display_name, "alex_dev");

        // "user_name" alias
        let json = r#"{"name":"Alex","user_name":"alex_dev2","password_hash":""}"#;
        let profile: UserProfile = serde_json::from_str(json).unwrap();
        assert_eq!(profile.display_name, "alex_dev2");
    }

    #[test]
    fn test_enum_internally_tagged() {
        let circle = Shape::Circle { radius: 5.0 };
        let json = serde_json::to_string(&circle).unwrap();
        assert!(json.contains(r#""type":"Circle"#));
        assert!(json.contains(r#""radius":5.0"#));

        let parsed: Shape = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, circle);
    }

    #[test]
    fn test_enum_externally_tagged() {
        let msg = Message::Text("hello".to_string());
        let json = serde_json::to_string(&msg).unwrap();
        assert!(json.contains("Text"));

        let parsed: Message = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, msg);
    }

    #[test]
    fn test_enum_untagged() {
        let int_val: FlexibleValue = serde_json::from_str("42").unwrap();
        assert_eq!(int_val, FlexibleValue::Integer(42));

        let str_val: FlexibleValue = serde_json::from_str(r#""hello""#).unwrap();
        assert_eq!(str_val, FlexibleValue::Text("hello".to_string()));

        let bool_val: FlexibleValue = serde_json::from_str("true").unwrap();
        assert_eq!(bool_val, FlexibleValue::Boolean(true));
    }

    #[test]
    fn test_dynamic_json() {
        let value = dynamic_json_example();
        assert_eq!(value["name"], "Alex");
        assert_eq!(value["age"], 25);
        assert_eq!(value["tags"][0], "rust");
        assert_eq!(value["address"]["city"], "Berlin");
    }

    #[test]
    fn test_navigate_dynamic_json() {
        let value = dynamic_json_example();
        let results = navigate_dynamic_json(&value);
        assert!(results.contains(&"name: Alex".to_string()));
        assert!(results.contains(&"age: 25".to_string()));
        assert!(results.contains(&"city: Berlin".to_string()));
    }

    #[test]
    fn test_modify_json() {
        let value = dynamic_json_example();
        let modified = modify_json(value);
        assert_eq!(modified["updated"], true);
        assert_eq!(modified["version"], 2);
        assert_eq!(modified["name"], "Alex"); // Original data preserved
    }

    #[test]
    fn test_flatten() {
        let json = flatten_example();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        // Fields are at the top level, not nested
        assert_eq!(parsed["query"], "rust");
        assert_eq!(parsed["page"], 1);
        assert_eq!(parsed["per_page"], 20);
        assert!(parsed.get("pagination").is_none());
    }

    #[test]
    fn test_hashmap_json() {
        let json = hashmap_json();
        let parsed: HashMap<String, i32> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.get("Alex"), Some(&95));
        assert_eq!(parsed.get("Sam"), Some(&87));
    }

    #[test]
    fn test_toml_roundtrip() {
        let (_, parsed) = toml_example();
        assert_eq!(parsed.title, "My App");
        assert!(parsed.debug);
        assert_eq!(parsed.database.max_connections, 5);
    }

    #[test]
    fn test_parse_toml() {
        let config = parse_toml_string();
        assert_eq!(config.title, "My App");
        assert!(!config.debug);
        assert_eq!(config.database.url, "postgres://localhost/mydb");
    }

    #[test]
    fn test_json_to_toml_conversion() {
        let json = r#"{"title":"App","debug":true,"database":{"url":"sqlite:db","max_connections":3}}"#;
        let toml_str = json_to_toml(json).unwrap();
        assert!(toml_str.contains("title = \"App\""));
    }

    #[test]
    fn test_toml_to_json_conversion() {
        let toml_str = r#"
            title = "App"
            debug = false
            [database]
            url = "sqlite:db"
            max_connections = 3
        "#;
        let json = toml_to_json(toml_str).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["title"], "App");
    }

    #[test]
    fn test_nested_structures() {
        let json = nested_structures();
        let team: Team = serde_json::from_str(&json).unwrap();
        assert_eq!(team.name, "Rust Team");
        assert_eq!(team.members.len(), 2);
        assert_eq!(team.tags.len(), 2);
    }

    #[test]
    fn test_pretty_print() {
        let user = User {
            name: "Alex".to_string(),
            email: "alex@example.com".to_string(),
            age: 25,
        };
        let pretty = serde_json::to_string_pretty(&user).unwrap();
        assert!(pretty.contains('\n'));
        assert!(pretty.contains("  ")); // Indentation
    }

    #[test]
    fn test_deserialize_extra_fields_ignored() {
        // Extra fields are ignored by default
        let json = r#"{"name":"Alex","email":"a@b.com","age":25,"extra":"ignored"}"#;
        let user: User = serde_json::from_str(json).unwrap();
        assert_eq!(user.name, "Alex");
    }

    #[test]
    fn test_deserialize_missing_field_error() {
        let json = r#"{"name":"Alex"}"#;
        let result: Result<User, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
