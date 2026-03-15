// Rust Tutorial #2: Installation and Your First Program
// https://kemalcodes.com/posts/rust-tutorial-installation/
//
// This file demonstrates:
// - fn main() — the entry point of every Rust program
// - println! macro — printing text to the terminal
// - let and let mut — creating variables
// - String::new() and read_line — reading user input
// - format! — string formatting
// - Basic functions with parameters and return values
// - Unit tests with #[test]

use std::io;

fn main() {
    // --- Hello World ---
    println!("Hello from Rust!");
    println!("This is Tutorial #2: Installation and First Program");
    println!();

    // --- Variables ---
    // let creates an immutable variable (cannot change)
    let language = "Rust";
    let year = 2026;
    println!("{} is the most admired language in {}!", language, year);

    // let mut creates a mutable variable (can change)
    let mut count = 0;
    count += 1;
    count += 1;
    println!("Count: {}", count);

    // --- String formatting ---
    let greeting = format!("Welcome to {} in {}", language, year);
    println!("{}", greeting);

    // --- Calling functions ---
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);

    let result = is_even(42);
    println!("Is 42 even? {}", result);

    let greeting = greet("Alex");
    println!("{}", greeting);

    // --- Interactive input ---
    println!();
    println!("What is your name?");

    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read input");

    let name = name.trim();
    println!("Hello, {}! You are now a Rust developer.", name);
}

// --- Functions ---

/// Adds two numbers and returns the result.
/// In Rust, the last expression without a semicolon is the return value.
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = this is the return value
}

/// Checks if a number is even.
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Creates a greeting message for the given name.
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to Rust.", name)
}

/// Calculates the factorial of a number.
/// Uses recursion — the function calls itself.
fn factorial(n: u64) -> u64 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

/// Returns the larger of two numbers.
fn max_of(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// --- Unit Tests ---
// Run with: cargo test

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(42));
        assert!(!is_even(1));
        assert!(!is_even(7));
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alex"), "Hello, Alex! Welcome to Rust.");
        assert_eq!(greet("Sam"), "Hello, Sam! Welcome to Rust.");
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(1), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(10), 3628800);
    }

    #[test]
    fn test_max_of() {
        assert_eq!(max_of(3, 5), 5);
        assert_eq!(max_of(10, 2), 10);
        assert_eq!(max_of(7, 7), 7);
        assert_eq!(max_of(-1, -5), -1);
    }
}
