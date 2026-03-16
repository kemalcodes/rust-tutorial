// Validation module
// Demonstrates a focused module with validation logic and custom error types.

use std::fmt;

/// Validation errors that can occur.
#[derive(Debug, PartialEq)]
pub enum ValidationError {
    TooShort { field: String, min: usize },
    TooLong { field: String, max: usize },
    InvalidFormat { field: String, message: String },
    Required { field: String },
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::TooShort { field, min } => {
                write!(f, "{} must be at least {} characters", field, min)
            }
            ValidationError::TooLong { field, max } => {
                write!(f, "{} must be at most {} characters", field, max)
            }
            ValidationError::InvalidFormat { field, message } => {
                write!(f, "{}: {}", field, message)
            }
            ValidationError::Required { field } => {
                write!(f, "{} is required", field)
            }
        }
    }
}

/// Validates that a string is not empty.
pub fn require(value: &str, field: &str) -> Result<(), ValidationError> {
    if value.trim().is_empty() {
        Err(ValidationError::Required {
            field: field.to_string(),
        })
    } else {
        Ok(())
    }
}

/// Validates string length is within range.
pub fn validate_length(
    value: &str,
    field: &str,
    min: usize,
    max: usize,
) -> Result<(), ValidationError> {
    let len = value.len();
    if len < min {
        Err(ValidationError::TooShort {
            field: field.to_string(),
            min,
        })
    } else if len > max {
        Err(ValidationError::TooLong {
            field: field.to_string(),
            max,
        })
    } else {
        Ok(())
    }
}

/// Validates an email address format.
pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    require(email, "email")?;

    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 || parts[0].is_empty() || !parts[1].contains('.') {
        return Err(ValidationError::InvalidFormat {
            field: "email".to_string(),
            message: "must be a valid email address".to_string(),
        });
    }
    Ok(())
}

/// Validates a username (alphanumeric, 3-20 chars).
pub fn validate_username(username: &str) -> Result<(), ValidationError> {
    require(username, "username")?;
    validate_length(username, "username", 3, 20)?;

    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(ValidationError::InvalidFormat {
            field: "username".to_string(),
            message: "must contain only letters, numbers, and underscores".to_string(),
        });
    }
    Ok(())
}

/// Collects multiple validation results into a list of errors.
pub fn collect_errors(results: Vec<Result<(), ValidationError>>) -> Vec<ValidationError> {
    results.into_iter().filter_map(|r| r.err()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_require() {
        assert!(require("hello", "name").is_ok());
        assert!(require("", "name").is_err());
        assert!(require("   ", "name").is_err());
    }

    #[test]
    fn test_validate_length() {
        assert!(validate_length("hello", "name", 3, 10).is_ok());
        assert!(validate_length("hi", "name", 3, 10).is_err());
        assert!(validate_length("hello world!", "name", 3, 10).is_err());
    }

    #[test]
    fn test_validate_email() {
        assert!(validate_email("alex@example.com").is_ok());
        assert!(validate_email("").is_err());
        assert!(validate_email("no-at-sign").is_err());
        assert!(validate_email("@example.com").is_err());
    }

    #[test]
    fn test_validate_username() {
        assert!(validate_username("alex_123").is_ok());
        assert!(validate_username("ab").is_err()); // Too short
        assert!(validate_username("alex@123").is_err()); // Invalid char
        assert!(validate_username("").is_err()); // Required
    }

    #[test]
    fn test_collect_errors() {
        let results = vec![
            require("", "name"),
            require("valid", "email"),
            validate_length("ab", "username", 3, 10),
        ];
        let errors = collect_errors(results);
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn test_validation_error_display() {
        let err = ValidationError::Required {
            field: "name".to_string(),
        };
        assert_eq!(format!("{}", err), "name is required");

        let err = ValidationError::TooShort {
            field: "password".to_string(),
            min: 8,
        };
        assert_eq!(format!("{}", err), "password must be at least 8 characters");
    }
}
