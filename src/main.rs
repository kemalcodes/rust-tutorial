// Rust Tutorial #3: Variables, Types, and Functions
// https://kemalcodes.com/posts/rust-tutorial-variables-types/
//
// This file demonstrates:
// - let (immutable) and let mut (mutable) variables
// - Constants with const
// - Data types: integers, floats, booleans, chars, strings
// - Type inference and explicit type annotations
// - Shadowing — reusing variable names with different values/types
// - Functions with parameters and return values
// - Expressions vs statements
// - Type conversions (as, parse, to_string)
// - String formatting (format!, Display vs Debug)
// - Tuples and arrays
// - Unit tests for every concept

// Constants — always typed, SCREAMING_SNAKE_CASE
const MAX_SCORE: u32 = 100;
const PI: f64 = 3.14159;
const APP_NAME: &str = "Rust Tutorial";

fn main() {
    println!("=== {} ===", APP_NAME);
    println!();

    // --- Immutable variables ---
    let language = "Rust";
    let year = 2026;
    println!("{} in {}", language, year);

    // --- Mutable variables ---
    let mut count = 0;
    count += 1;
    count += 1;
    count += 1;
    println!("Count: {}", count);

    // --- Type annotations ---
    let age: u32 = 25;
    let temperature: f64 = 36.6;
    let active: bool = true;
    let grade: char = 'A';
    println!("Age: {}, Temp: {}, Active: {}, Grade: {}", age, temperature, active, grade);

    // --- Shadowing ---
    let x = 5;
    let x = x + 1;      // New variable, shadows the old one
    let x = x * 2;      // Again
    println!("Shadowed x: {}", x);  // 12

    // Shadowing can change types
    let input = "42";
    let input: i32 = input.parse().unwrap();
    println!("Parsed input: {}", input);

    // --- Strings ---
    let greeting: &str = "Hello";                  // String slice (borrowed)
    let mut name = String::from("Alex");           // Owned string
    name.push_str(" Smith");
    println!("{}, {}!", greeting, name);

    // --- Formatting ---
    let message = format!("{} scored {} out of {}", "Sam", 95, MAX_SCORE);
    println!("{}", message);
    println!("PI = {:.2}", PI);  // 2 decimal places

    // Debug formatting
    let scores = [85, 92, 78, 95, 88];
    println!("Scores: {:?}", scores);

    // --- Tuples ---
    let person = ("Jordan", 30, true);
    let (p_name, p_age, p_active) = person;
    println!("{} is {} years old, active: {}", p_name, p_age, p_active);

    // --- Functions ---
    println!("add(10, 20) = {}", add(10, 20));
    println!("multiply(6, 7) = {}", multiply(6, 7));
    println!("is_adult(15) = {}", is_adult(15));
    println!("is_adult(21) = {}", is_adult(21));
    println!("max_of(3, 7) = {}", max_of(3, 7));
    println!("celsius_to_fahrenheit(100.0) = {:.1}", celsius_to_fahrenheit(100.0));

    let (min, max) = min_max(&scores);
    println!("Min: {}, Max: {}", min, max);

    // --- Type conversions ---
    let int_val: i32 = 42;
    let float_val: f64 = int_val as f64;
    let small_val: u8 = int_val as u8;
    println!("{} as f64 = {}, as u8 = {}", int_val, float_val, small_val);

    // --- Expressions ---
    let status = if 95 > 50 { "pass" } else { "fail" };
    println!("Status: {}", status);

    let block_result = {
        let a = 5;
        let b = 10;
        a + b  // Block returns this value
    };
    println!("Block result: {}", block_result);
}

// --- Functions ---

/// Adds two numbers. Last expression without semicolon is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Multiplies two numbers.
fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// Checks if a person is an adult (18 or older).
fn is_adult(age: u32) -> bool {
    age >= 18
}

/// Returns the larger of two numbers.
fn max_of(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

/// Converts Celsius to Fahrenheit.
fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Returns the minimum and maximum values from a slice.
fn min_max(numbers: &[i32]) -> (i32, i32) {
    let min = *numbers.iter().min().unwrap();
    let max = *numbers.iter().max().unwrap();
    (min, max)
}

/// Divides two numbers, returning 0.0 if divisor is zero.
fn safe_divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        return 0.0;  // Early return
    }
    a / b
}

/// Describes a score as a grade string.
fn grade_from_score(score: u32) -> &'static str {
    if score >= 90 { "A" }
    else if score >= 80 { "B" }
    else if score >= 70 { "C" }
    else if score >= 60 { "D" }
    else { "F" }
}

/// Checks if a string is a valid positive integer.
fn is_valid_number(text: &str) -> bool {
    text.parse::<u64>().is_ok()
}

// --- Unit Tests ---
// Run with: cargo test

#[cfg(test)]
mod tests {
    use super::*;

    // --- Arithmetic tests ---

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
        assert_eq!(add(100, 200), 300);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(3, 4), 12);
        assert_eq!(multiply(-2, 5), -10);
        assert_eq!(multiply(0, 100), 0);
    }

    // --- Boolean logic tests ---

    #[test]
    fn test_is_adult() {
        assert!(!is_adult(0));
        assert!(!is_adult(17));
        assert!(is_adult(18));
        assert!(is_adult(21));
        assert!(is_adult(100));
    }

    // --- Comparison tests ---

    #[test]
    fn test_max_of() {
        assert_eq!(max_of(3, 5), 5);
        assert_eq!(max_of(10, 2), 10);
        assert_eq!(max_of(7, 7), 7);
        assert_eq!(max_of(-3, -1), -1);
    }

    // --- Conversion tests ---

    #[test]
    fn test_celsius_to_fahrenheit() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
        assert!((celsius_to_fahrenheit(37.0) - 98.6).abs() < 0.01);
    }

    // --- Tuple return tests ---

    #[test]
    fn test_min_max() {
        assert_eq!(min_max(&[1, 2, 3, 4, 5]), (1, 5));
        assert_eq!(min_max(&[5, 3, 1, 4, 2]), (1, 5));
        assert_eq!(min_max(&[42]), (42, 42));
        assert_eq!(min_max(&[-5, -1, -10]), (-10, -1));
    }

    // --- Division tests ---

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10.0, 2.0), 5.0);
        assert_eq!(safe_divide(10.0, 0.0), 0.0);  // Division by zero returns 0
        assert_eq!(safe_divide(0.0, 5.0), 0.0);
    }

    // --- Grade tests ---

    #[test]
    fn test_grade_from_score() {
        assert_eq!(grade_from_score(95), "A");
        assert_eq!(grade_from_score(90), "A");
        assert_eq!(grade_from_score(85), "B");
        assert_eq!(grade_from_score(75), "C");
        assert_eq!(grade_from_score(65), "D");
        assert_eq!(grade_from_score(50), "F");
        assert_eq!(grade_from_score(0), "F");
    }

    // --- Validation tests ---

    #[test]
    fn test_is_valid_number() {
        assert!(is_valid_number("42"));
        assert!(is_valid_number("0"));
        assert!(is_valid_number("999999"));
        assert!(!is_valid_number("abc"));
        assert!(!is_valid_number(""));
        assert!(!is_valid_number("-5"));  // Negative — not valid for u64
        assert!(!is_valid_number("3.14")); // Float — not valid for u64
    }

    // --- Constants tests ---

    #[test]
    fn test_constants() {
        assert_eq!(MAX_SCORE, 100);
        assert!((PI - 3.14159).abs() < 0.00001);
        assert_eq!(APP_NAME, "Rust Tutorial");
    }

    // --- Shadowing test ---

    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        assert_eq!(x, 12);

        // Shadowing can change types
        let value = "100";
        let value: i32 = value.parse().unwrap();
        assert_eq!(value, 100);
    }
}
