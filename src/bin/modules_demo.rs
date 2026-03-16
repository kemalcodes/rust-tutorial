// Rust Tutorial #20: Modules, Crates, and Project Organization
// Demonstrates mod, pub, use, file-based modules, and re-exports.
// This binary shows how to USE the library modules defined in lib.rs.

// Import from our own crate (rust_tutorial is the crate name from Cargo.toml)
use rust_tutorial::math;
use rust_tutorial::utils;
use rust_tutorial::validation;
use rust_tutorial::{Task, TaskStatus, User};

// --- Inline module (defined in the same file) ---

mod config {
    /// Application configuration.
    pub struct AppConfig {
        pub app_name: String,
        pub version: String,
        pub debug: bool,
    }

    impl AppConfig {
        pub fn new() -> Self {
            Self {
                app_name: "Rust Tutorial App".to_string(),
                version: "1.0.0".to_string(),
                debug: false,
            }
        }

        pub fn with_debug(mut self) -> Self {
            self.debug = true;
            self
        }
    }

    impl Default for AppConfig {
        fn default() -> Self {
            Self::new()
        }
    }

    // Private helper — not accessible outside this module
    #[allow(dead_code)]
    fn internal_helper() -> &'static str {
        "this is private"
    }

    // Nested module
    pub mod defaults {
        pub const MAX_RETRIES: u32 = 3;
        pub const TIMEOUT_SECS: u64 = 30;
    }
}

// --- Using a nested module ---

mod services {
    // Import from parent module
    use super::config::AppConfig;

    pub struct Logger {
        prefix: String,
    }

    impl Logger {
        pub fn new(config: &AppConfig) -> Self {
            Self {
                prefix: config.app_name.clone(),
            }
        }

        pub fn log(&self, message: &str) -> String {
            format!("[{}] {}", self.prefix, message)
        }
    }
}

fn main() {
    println!("=== Module Demo ===\n");

    // --- Using the math module ---
    println!("--- Math Module ---");
    println!("add(2, 3) = {}", math::add(2, 3));
    println!("multiply(4, 5) = {}", math::multiply(4, 5));
    println!("divide(10, 3) = {:?}", math::divide(10.0, 3.0));
    println!("gcd(12, 8) = {}", math::gcd(12, 8));
    println!("average([1,2,3]) = {:?}", math::average(&[1.0, 2.0, 3.0]));
    println!("clamp(15, 0, 10) = {}", math::clamp(15, 0, 10));

    // --- Using the models module ---
    println!("\n--- Models Module ---");
    let user = User::new("Alex", "alex@example.com", 25);
    println!("User: {}", user);
    println!("Is adult: {}", user.is_adult());
    // println!("Age: {}", user.age);  // ERROR: age is private
    println!("Age: {}", user.age()); // OK: using public getter

    let mut task = Task::new("Write documentation");
    println!("Task: {}", task);
    task.assign_to(user.clone());
    println!("After assign: {}", task);
    task.complete();
    println!("After complete: {}", task);

    // --- Using the utils module ---
    println!("\n--- Utils Module ---");
    println!("capitalize: {}", utils::capitalize("hello world"));
    println!("truncate: {}", utils::truncate("Hello, World!", 8));
    println!("slugify: {}", utils::slugify("Rust Tutorial #20!"));
    println!(
        "repeat: {}",
        utils::repeat_with_separator("ha", 3, " ")
    );
    println!(
        "email valid: {}",
        utils::is_valid_email("alex@example.com")
    );

    // --- Using the validation module ---
    println!("\n--- Validation Module ---");
    match validation::validate_email("alex@example.com") {
        Ok(()) => println!("Email is valid"),
        Err(e) => println!("Email error: {}", e),
    }
    match validation::validate_username("ab") {
        Ok(()) => println!("Username is valid"),
        Err(e) => println!("Username error: {}", e),
    }

    // Collect multiple validation errors
    let errors = validation::collect_errors(vec![
        validation::require("", "name"),
        validation::validate_email("bad-email"),
        validation::validate_username("alex_123"),
    ]);
    println!("Validation errors: {}", errors.len());
    for err in &errors {
        println!("  - {}", err);
    }

    // --- Using inline modules ---
    println!("\n--- Config Module ---");
    let config = config::AppConfig::new().with_debug();
    println!("App: {} v{}", config.app_name, config.version);
    println!("Debug: {}", config.debug);
    println!("Max retries: {}", config::defaults::MAX_RETRIES);
    println!("Timeout: {}s", config::defaults::TIMEOUT_SECS);

    // --- Using services module ---
    println!("\n--- Services Module ---");
    let logger = services::Logger::new(&config);
    println!("{}", logger.log("Application started"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_operations() {
        assert_eq!(math::add(2, 3), 5);
        assert_eq!(math::multiply(4, 5), 20);
        assert_eq!(math::gcd(12, 8), 4);
    }

    #[test]
    fn test_user_creation() {
        let user = User::new("Alex", "alex@example.com", 25);
        assert_eq!(user.name, "Alex");
        assert!(user.is_adult());
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new("Test");
        assert_eq!(task.status, TaskStatus::Pending);

        let user = User::new("Sam", "sam@example.com", 30);
        task.assign_to(user);
        assert_eq!(task.status, TaskStatus::InProgress);

        task.complete();
        assert!(task.is_done());
    }

    #[test]
    fn test_utils() {
        assert_eq!(utils::capitalize("hello"), "Hello");
        assert_eq!(utils::slugify("Hello World!"), "hello-world");
    }

    #[test]
    fn test_validation() {
        assert!(validation::validate_email("alex@example.com").is_ok());
        assert!(validation::validate_email("bad").is_err());
        assert!(validation::validate_username("alex_123").is_ok());
        assert!(validation::validate_username("ab").is_err());
    }

    #[test]
    fn test_inline_module() {
        let config = config::AppConfig::new();
        assert_eq!(config.app_name, "Rust Tutorial App");
        assert!(!config.debug);

        let debug_config = config.with_debug();
        assert!(debug_config.debug);
    }

    #[test]
    fn test_nested_module_constants() {
        assert_eq!(config::defaults::MAX_RETRIES, 3);
        assert_eq!(config::defaults::TIMEOUT_SECS, 30);
    }

    #[test]
    fn test_services_logger() {
        let config = config::AppConfig::new();
        let logger = services::Logger::new(&config);
        let msg = logger.log("test");
        assert_eq!(msg, "[Rust Tutorial App] test");
    }

    #[test]
    fn test_re_exports() {
        // Test that re-exports work — we can import Task directly
        let task = Task::new("Test re-export");
        assert_eq!(task.title, "Test re-export");
    }
}
