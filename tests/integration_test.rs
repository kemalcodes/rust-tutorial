// Rust Tutorial #21: Integration Tests
// Integration tests live in the tests/ directory and test the public API.
// Each file in tests/ is compiled as a separate crate.

// Import from the library crate
use rust_tutorial::math;
use rust_tutorial::modules::validation;
use rust_tutorial::utils;
use rust_tutorial::{Task, TaskStatus, User};

// --- Math module integration tests ---

#[test]
fn test_math_operations_together() {
    let sum = math::add(10, 20);
    let product = math::multiply(sum, 2);
    assert_eq!(product, 60);
}

#[test]
fn test_math_divide_and_average() {
    let result = math::divide(10.0, 3.0).unwrap();
    assert!((result - 3.333).abs() < 0.01);

    let avg = math::average(&[1.0, 2.0, 3.0, 4.0, 5.0]).unwrap();
    assert_eq!(avg, 3.0);
}

#[test]
fn test_math_gcd() {
    assert_eq!(math::gcd(12, 8), 4);
    assert_eq!(math::gcd(100, 75), 25);
    assert_eq!(math::gcd(7, 13), 1);
}

// --- Models integration tests ---

#[test]
fn test_user_and_task_workflow() {
    let user = User::new("Alex", "alex@example.com", 25);

    let mut task = Task::new("Review code");
    assert_eq!(task.status, TaskStatus::Pending);

    task.assign_to(user);
    assert_eq!(task.status, TaskStatus::InProgress);
    assert!(task.assignee.is_some());

    task.complete();
    assert!(task.is_done());
}

#[test]
fn test_multiple_tasks() {
    let user = User::new("Sam", "sam@example.com", 30);

    let mut tasks: Vec<Task> = (1..=5)
        .map(|i| Task::new(&format!("Task {}", i)))
        .collect();

    // Assign all tasks to the user
    for task in &mut tasks {
        task.assign_to(user.clone());
    }

    // Complete some tasks
    tasks[0].complete();
    tasks[2].complete();

    let completed: Vec<&Task> = tasks.iter().filter(|t| t.is_done()).collect();
    assert_eq!(completed.len(), 2);

    let in_progress: Vec<&Task> = tasks
        .iter()
        .filter(|t| t.status == TaskStatus::InProgress)
        .collect();
    assert_eq!(in_progress.len(), 3);
}

// --- Utils integration tests ---

#[test]
fn test_utils_string_processing() {
    let title = "hello world rust tutorial";
    let capitalized = utils::capitalize(title);
    let slug = utils::slugify(title);

    assert_eq!(capitalized, "Hello world rust tutorial");
    assert_eq!(slug, "hello-world-rust-tutorial");
}

#[test]
fn test_utils_truncate() {
    let long_text = "This is a very long string that needs truncation";
    let short = utils::truncate(long_text, 20);
    assert!(short.len() <= 20);
    assert!(short.ends_with("..."));
}

// --- Validation integration tests ---

#[test]
fn test_validation_workflow() {
    // Valid data
    assert!(validation::validate_email("alex@example.com").is_ok());
    assert!(validation::validate_username("alex_123").is_ok());

    // Invalid data
    assert!(validation::validate_email("not-an-email").is_err());
    assert!(validation::validate_username("ab").is_err());

    // Collect all errors at once
    let errors = validation::collect_errors(vec![
        validation::require("", "name"),
        validation::validate_email("bad"),
        validation::validate_username("x"),
    ]);
    assert_eq!(errors.len(), 3);
}

#[test]
fn test_validation_length() {
    assert!(validation::validate_length("hello", "field", 3, 10).is_ok());
    assert!(validation::validate_length("hi", "field", 3, 10).is_err());
    assert!(validation::validate_length("this is too long", "field", 3, 10).is_err());
}

// --- Cross-module integration tests ---

#[test]
fn test_user_with_validation() {
    let name = "alex_123";
    let email = "alex@example.com";

    // Validate before creating
    assert!(validation::validate_username(name).is_ok());
    assert!(validation::validate_email(email).is_ok());

    // Create the user
    let user = User::new("Alex", email, 25);
    assert_eq!(user.name, "Alex");
    assert!(user.is_adult());
}

#[test]
fn test_slugify_and_capitalize_roundtrip() {
    let input = "Rust Tutorial #21: Testing!";
    let slug = utils::slugify(input);
    assert_eq!(slug, "rust-tutorial-21-testing");

    // Capitalize the slug (not a true roundtrip, just showing composition)
    let cap = utils::capitalize(&slug);
    assert_eq!(cap, "Rust-tutorial-21-testing");
}
