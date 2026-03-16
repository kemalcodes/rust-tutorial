// Utility functions module
// Demonstrates string helpers, formatting, and general-purpose utilities.

/// Capitalizes the first letter of a string.
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

/// Truncates a string to the given length, adding "..." if truncated.
pub fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else if max_len <= 3 {
        s[..max_len].to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

/// Slugifies a string (lowercase, spaces to hyphens, remove special chars).
pub fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() || c == ' ' { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join("-")
}

/// Repeats a string n times with a separator.
pub fn repeat_with_separator(s: &str, n: usize, sep: &str) -> String {
    let parts: Vec<&str> = (0..n).map(|_| s).collect();
    parts.join(sep)
}

/// Checks if a string is a valid email (simple check).
pub fn is_valid_email(email: &str) -> bool {
    let parts: Vec<&str> = email.split('@').collect();
    if parts.len() != 2 {
        return false;
    }
    let local = parts[0];
    let domain = parts[1];
    !local.is_empty() && domain.contains('.') && domain.len() > 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!(capitalize("hello"), "Hello");
        assert_eq!(capitalize(""), "");
        assert_eq!(capitalize("a"), "A");
        assert_eq!(capitalize("Hello"), "Hello");
    }

    #[test]
    fn test_truncate() {
        assert_eq!(truncate("Hello, World!", 5), "He...");
        assert_eq!(truncate("Hi", 10), "Hi");
        assert_eq!(truncate("Hello", 5), "Hello");
        assert_eq!(truncate("Hello", 3), "Hel");
    }

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World!"), "hello-world");
        assert_eq!(slugify("Rust Tutorial #20"), "rust-tutorial-20");
        assert_eq!(slugify("  spaces  everywhere  "), "spaces-everywhere");
    }

    #[test]
    fn test_repeat_with_separator() {
        assert_eq!(repeat_with_separator("ha", 3, " "), "ha ha ha");
        assert_eq!(repeat_with_separator("x", 2, "-"), "x-x");
    }

    #[test]
    fn test_is_valid_email() {
        assert!(is_valid_email("alex@example.com"));
        assert!(is_valid_email("user@a.co"));
        assert!(!is_valid_email("no-at-sign"));
        assert!(!is_valid_email("@example.com"));
        assert!(!is_valid_email("user@"));
        assert!(!is_valid_email("user@a"));
    }
}
