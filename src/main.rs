// Rust Tutorial #8: Error Handling
// Demonstrates Result<T, E>, Option<T>, the ? operator, unwrap/expect,
// custom error types, From trait, and combinators.

use std::fmt;
use std::num::ParseIntError;

// --- Custom Error Type ---
#[derive(Debug, PartialEq)]
enum AppError {
    InvalidInput(String),
    ParseFailed(ParseIntError),
    DivisionByZero,
    OutOfRange { value: i32, min: i32, max: i32 },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            AppError::ParseFailed(e) => write!(f, "Parse error: {}", e),
            AppError::DivisionByZero => write!(f, "Division by zero"),
            AppError::OutOfRange { value, min, max } => {
                write!(f, "Value {} is out of range [{}, {}]", value, min, max)
            }
        }
    }
}

// --- From trait for automatic conversion with ? ---
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> AppError {
        AppError::ParseFailed(e)
    }
}

// --- Functions returning Result ---

/// Divides two numbers, returns error on division by zero.
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}

/// Parses a string to i32 and doubles it, using the ? operator.
fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
    let number = input.parse::<i32>()?;
    Ok(number * 2)
}

/// Adds two string numbers together, using ? for both parses.
fn add_strings(a: &str, b: &str) -> Result<i32, ParseIntError> {
    let x = a.parse::<i32>()?;
    let y = b.parse::<i32>()?;
    Ok(x + y)
}

/// Parses age with custom error type and range validation.
fn parse_age(input: &str) -> Result<i32, AppError> {
    let age = input.parse::<i32>()?; // Uses From<ParseIntError>
    if age < 0 || age > 150 {
        return Err(AppError::OutOfRange {
            value: age,
            min: 0,
            max: 150,
        });
    }
    Ok(age)
}

/// Safe division with custom error type.
fn safe_divide(a: i32, b: i32) -> Result<i32, AppError> {
    if b == 0 {
        Err(AppError::DivisionByZero)
    } else {
        Ok(a / b)
    }
}

// --- Option<T> functions ---

/// Finds the first even number in a slice.
fn first_even(numbers: &[i32]) -> Option<i32> {
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);
        }
    }
    None
}

/// Finds an item in a list by name.
fn find_user(users: &[&str], target: &str) -> Option<usize> {
    for (i, &user) in users.iter().enumerate() {
        if user == target {
            return Some(i);
        }
    }
    None
}

// --- Combinator examples ---

/// Parses a string to a positive u32 using map_err and and_then.
fn parse_positive(input: &str) -> Result<u32, String> {
    input
        .parse::<i32>()
        .map_err(|e| e.to_string())
        .and_then(|n| {
            if n > 0 {
                Ok(n as u32)
            } else {
                Err(String::from("must be positive"))
            }
        })
}

/// Uses map to transform a Result value.
fn double_result(input: &str) -> Result<i32, ParseIntError> {
    input.parse::<i32>().map(|n| n * 2)
}

/// Uses unwrap_or_else to provide a default on error.
fn parse_or_default(input: &str) -> i32 {
    input.parse::<i32>().unwrap_or_else(|_| 0)
}

/// Converts Option to Result using ok_or.
fn require_even(numbers: &[i32]) -> Result<i32, String> {
    first_even(numbers).ok_or(String::from("no even number found"))
}

fn main() {
    // --- Result with match ---
    println!("--- divide ---");
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }
    match divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {:.2}", result),
        Err(e) => println!("Error: {}", e),
    }

    // --- The ? operator ---
    println!("\n--- parse_and_double ---");
    println!("'21' => {:?}", parse_and_double("21"));
    println!("'abc' => {:?}", parse_and_double("abc"));

    // --- Custom error ---
    println!("\n--- parse_age ---");
    println!("'25' => {:?}", parse_age("25"));
    println!("'200' => {:?}", parse_age("200"));
    println!("'xyz' => {:?}", parse_age("xyz"));

    // --- Option ---
    println!("\n--- first_even ---");
    println!("[1,3,5]: {:?}", first_even(&[1, 3, 5]));
    println!("[1,4,5]: {:?}", first_even(&[1, 4, 5]));

    // --- Combinators ---
    println!("\n--- combinators ---");
    println!("parse_positive('42'): {:?}", parse_positive("42"));
    println!("parse_positive('-5'): {:?}", parse_positive("-5"));
    println!("parse_or_default('abc'): {}", parse_or_default("abc"));

    // --- unwrap and expect (safe usage) ---
    let number: i32 = "42".parse().expect("hardcoded value must parse");
    println!("\nParsed with expect: {}", number);
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: divide returns Ok for valid division
    #[test]
    fn test_divide_ok() {
        let result = divide(10.0, 4.0);
        assert!(result.is_ok());
        assert!((result.unwrap() - 2.5).abs() < f64::EPSILON);
    }

    // Test 2: divide returns Err for division by zero
    #[test]
    fn test_divide_by_zero() {
        let result = divide(10.0, 0.0);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "cannot divide by zero");
    }

    // Test 3: parse_and_double with valid input
    #[test]
    fn test_parse_and_double_valid() {
        assert_eq!(parse_and_double("21"), Ok(42));
    }

    // Test 4: parse_and_double with invalid input
    #[test]
    fn test_parse_and_double_invalid() {
        assert!(parse_and_double("abc").is_err());
    }

    // Test 5: add_strings with valid inputs
    #[test]
    fn test_add_strings_valid() {
        assert_eq!(add_strings("10", "20"), Ok(30));
    }

    // Test 6: add_strings with one invalid input
    #[test]
    fn test_add_strings_invalid() {
        assert!(add_strings("10", "xyz").is_err());
        assert!(add_strings("abc", "20").is_err());
    }

    // Test 7: parse_age valid age
    #[test]
    fn test_parse_age_valid() {
        assert_eq!(parse_age("25"), Ok(25));
        assert_eq!(parse_age("0"), Ok(0));
        assert_eq!(parse_age("150"), Ok(150));
    }

    // Test 8: parse_age out of range
    #[test]
    fn test_parse_age_out_of_range() {
        let result = parse_age("200");
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            AppError::OutOfRange { value: 200, min: 0, max: 150 }
        );
    }

    // Test 9: parse_age invalid string (From trait conversion)
    #[test]
    fn test_parse_age_invalid_string() {
        let result = parse_age("xyz");
        assert!(result.is_err());
        match result.unwrap_err() {
            AppError::ParseFailed(_) => {} // Expected
            other => panic!("Expected ParseFailed, got {:?}", other),
        }
    }

    // Test 10: safe_divide ok and error
    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Ok(5));
        assert_eq!(safe_divide(10, 0), Err(AppError::DivisionByZero));
    }

    // Test 11: first_even finds even number
    #[test]
    fn test_first_even_found() {
        assert_eq!(first_even(&[1, 3, 4, 6]), Some(4));
    }

    // Test 12: first_even returns None when no even
    #[test]
    fn test_first_even_none() {
        assert_eq!(first_even(&[1, 3, 5, 7]), None);
    }

    // Test 13: find_user found and not found
    #[test]
    fn test_find_user() {
        let users = ["Alex", "Sam", "Jordan"];
        assert_eq!(find_user(&users, "Sam"), Some(1));
        assert_eq!(find_user(&users, "Unknown"), None);
    }

    // Test 14: parse_positive with combinators
    #[test]
    fn test_parse_positive() {
        assert_eq!(parse_positive("42"), Ok(42));
        assert_eq!(parse_positive("-5"), Err(String::from("must be positive")));
        assert!(parse_positive("abc").is_err());
    }

    // Test 15: double_result with map combinator
    #[test]
    fn test_double_result() {
        assert_eq!(double_result("5"), Ok(10));
        assert!(double_result("abc").is_err());
    }

    // Test 16: parse_or_default with unwrap_or_else
    #[test]
    fn test_parse_or_default() {
        assert_eq!(parse_or_default("42"), 42);
        assert_eq!(parse_or_default("abc"), 0);
    }

    // Test 17: ok_or converts Option to Result
    #[test]
    fn test_require_even() {
        assert_eq!(require_even(&[1, 2, 3]), Ok(2));
        assert_eq!(
            require_even(&[1, 3, 5]),
            Err(String::from("no even number found"))
        );
    }

    // Test 18: AppError Display formatting
    #[test]
    fn test_app_error_display() {
        let err = AppError::DivisionByZero;
        assert_eq!(format!("{}", err), "Division by zero");

        let err = AppError::InvalidInput(String::from("empty"));
        assert_eq!(format!("{}", err), "Invalid input: empty");
    }
}
