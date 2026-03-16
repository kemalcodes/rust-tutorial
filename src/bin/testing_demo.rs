// Rust Tutorial #21: Testing in Rust
// Demonstrates #[test], assert macros, #[should_panic], Result-returning tests,
// #[ignore], test helpers, and test organization.

// --- Code to test ---

/// Adds two numbers.
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Divides a by b. Returns None if b is zero.
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Parses a string to i32.
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|e| format!("Parse error: {}", e))
}

/// Returns the first element of a slice, or None if empty.
pub fn first<T: Clone>(items: &[T]) -> Option<T> {
    items.first().cloned()
}

/// Checks if a string is a palindrome.
pub fn is_palindrome(s: &str) -> bool {
    let cleaned: String = s.chars().filter(|c| c.is_alphanumeric()).collect();
    let lower = cleaned.to_lowercase();
    lower == lower.chars().rev().collect::<String>()
}

/// Capitalizes the first letter.
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => {
            let upper: String = first.to_uppercase().collect();
            upper + chars.as_str()
        }
    }
}

/// A simple user struct for testing.
#[derive(Debug, PartialEq, Clone)]
pub struct User {
    pub name: String,
    pub age: u32,
}

impl User {
    pub fn new(name: &str, age: u32) -> Result<Self, String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        if age > 150 {
            return Err("Age must be 150 or less".to_string());
        }
        Ok(Self {
            name: name.trim().to_string(),
            age,
        })
    }

    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

/// Panics if the value is negative.
pub fn assert_positive(value: i32) -> i32 {
    if value < 0 {
        panic!("Value must be positive, got {}", value);
    }
    value
}

/// A container that validates capacity.
pub struct BoundedVec {
    items: Vec<String>,
    capacity: usize,
}

impl BoundedVec {
    pub fn new(capacity: usize) -> Self {
        Self {
            items: Vec::new(),
            capacity,
        }
    }

    pub fn push(&mut self, item: String) -> Result<(), String> {
        if self.items.len() >= self.capacity {
            Err(format!("Container is full (max {})", self.capacity))
        } else {
            self.items.push(item);
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.items.get(index).map(|s| s.as_str())
    }

    pub fn contains(&self, item: &str) -> bool {
        self.items.iter().any(|s| s == item)
    }
}

fn main() {
    println!("Run `cargo test --bin testing_demo` to see the tests in action!");
    println!("add(2, 3) = {}", add(2, 3));

    let user = User::new("Alex", 25).unwrap();
    println!("User: {:?}", user);
    println!("Greeting: {}", user.greet());
    println!("Is palindrome 'racecar': {}", is_palindrome("racecar"));
}

// ==========================================================
// UNIT TESTS — in the same file, inside #[cfg(test)]
// ==========================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- Basic assert macros ---

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_add_negative() {
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(-5, -3), -8);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    // --- assert_ne! macro ---

    #[test]
    fn test_add_not_equal() {
        assert_ne!(add(2, 3), 6);
    }

    // --- assert! for boolean conditions ---

    #[test]
    fn test_divide_some() {
        let result = divide(10.0, 3.0);
        assert!(result.is_some());
        // Check approximate equality for floats
        let value = result.unwrap();
        assert!((value - 3.333).abs() < 0.01, "Expected ~3.333, got {}", value);
    }

    #[test]
    fn test_divide_by_zero() {
        assert!(divide(10.0, 0.0).is_none());
    }

    // --- Custom failure messages ---

    #[test]
    fn test_add_with_message() {
        let result = add(2, 3);
        assert_eq!(
            result, 5,
            "Expected 2 + 3 to equal 5, but got {}",
            result
        );
    }

    // --- Testing Option and Result ---

    #[test]
    fn test_parse_number_ok() {
        let result = parse_number("42");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_parse_number_err() {
        let result = parse_number("abc");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.contains("Parse error"), "Unexpected error: {}", err);
    }

    #[test]
    fn test_first_some() {
        assert_eq!(first(&[1, 2, 3]), Some(1));
    }

    #[test]
    fn test_first_none() {
        let empty: Vec<i32> = vec![];
        assert_eq!(first(&empty), None);
    }

    // --- Result-returning tests ---
    // Instead of unwrap(), return Result so the test fails with a clear message.

    #[test]
    fn test_parse_returns_result() -> Result<(), String> {
        let value = parse_number("42")?;
        assert_eq!(value, 42);
        Ok(())
    }

    // --- #[should_panic] ---

    #[test]
    #[should_panic]
    fn test_assert_positive_panics() {
        assert_positive(-1);
    }

    #[test]
    #[should_panic(expected = "Value must be positive")]
    fn test_assert_positive_panic_message() {
        assert_positive(-5);
    }

    #[test]
    fn test_assert_positive_ok() {
        assert_eq!(assert_positive(5), 5);
        assert_eq!(assert_positive(0), 0);
    }

    // --- Testing structs ---

    #[test]
    fn test_user_creation() {
        let user = User::new("Alex", 25).unwrap();
        assert_eq!(user.name, "Alex");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_user_empty_name() {
        let result = User::new("", 25);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Name cannot be empty");
    }

    #[test]
    fn test_user_whitespace_name() {
        let result = User::new("   ", 25);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_invalid_age() {
        let result = User::new("Alex", 200);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("150"));
    }

    #[test]
    fn test_user_name_trimmed() {
        let user = User::new("  Alex  ", 25).unwrap();
        assert_eq!(user.name, "Alex");
    }

    #[test]
    fn test_user_is_adult() {
        assert!(User::new("Alex", 18).unwrap().is_adult());
        assert!(User::new("Sam", 30).unwrap().is_adult());
        assert!(!User::new("Jordan", 17).unwrap().is_adult());
    }

    #[test]
    fn test_user_greet() {
        let user = User::new("Sam", 20).unwrap();
        assert_eq!(user.greet(), "Hello, Sam!");
    }

    #[test]
    fn test_user_equality() {
        let user1 = User::new("Alex", 25).unwrap();
        let user2 = User::new("Alex", 25).unwrap();
        assert_eq!(user1, user2);
    }

    #[test]
    fn test_user_clone() {
        let user1 = User::new("Alex", 25).unwrap();
        let user2 = user1.clone();
        assert_eq!(user1, user2);
    }

    // --- Testing string functions ---

    #[test]
    fn test_is_palindrome() {
        assert!(is_palindrome("racecar"));
        assert!(is_palindrome("A man a plan a canal Panama"));
        assert!(is_palindrome(""));
        assert!(is_palindrome("a"));
        assert!(!is_palindrome("hello"));
    }

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("Hello"), "Hello");
    }

    // --- Testing BoundedVec ---

    #[test]
    fn test_bounded_vec_basic() {
        let mut v = BoundedVec::new(3);
        assert!(v.is_empty());
        assert_eq!(v.len(), 0);

        v.push("one".to_string()).unwrap();
        assert_eq!(v.len(), 1);
        assert!(!v.is_empty());
    }

    #[test]
    fn test_bounded_vec_capacity() {
        let mut v = BoundedVec::new(2);
        v.push("one".to_string()).unwrap();
        v.push("two".to_string()).unwrap();

        let result = v.push("three".to_string());
        assert!(result.is_err());
        assert_eq!(v.len(), 2);
    }

    #[test]
    fn test_bounded_vec_get() {
        let mut v = BoundedVec::new(5);
        v.push("hello".to_string()).unwrap();

        assert_eq!(v.get(0), Some("hello"));
        assert_eq!(v.get(1), None);
    }

    #[test]
    fn test_bounded_vec_contains() {
        let mut v = BoundedVec::new(5);
        v.push("rust".to_string()).unwrap();

        assert!(v.contains("rust"));
        assert!(!v.contains("python"));
    }

    // --- #[ignore] for slow tests ---

    #[test]
    #[ignore]
    fn test_slow_operation() {
        // This test is skipped by default.
        // Run with: cargo test -- --ignored
        // Run all tests including ignored: cargo test -- --include-ignored
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(true);
    }

    // --- Test helper functions ---

    fn create_test_user() -> User {
        User::new("Test User", 25).unwrap()
    }

    fn create_test_users(count: usize) -> Vec<User> {
        (0..count)
            .map(|i| User::new(&format!("User {}", i), 20 + i as u32).unwrap())
            .collect()
    }

    #[test]
    fn test_with_helper() {
        let user = create_test_user();
        assert_eq!(user.name, "Test User");
    }

    #[test]
    fn test_with_multiple_users() {
        let users = create_test_users(3);
        assert_eq!(users.len(), 3);
        assert_eq!(users[0].name, "User 0");
        assert_eq!(users[2].age, 22);
    }
}
