// Math utilities module
// Demonstrates public functions, private helpers, and module organization.

/// Adds two numbers.
pub fn add(a: i64, b: i64) -> i64 {
    a + b
}

/// Subtracts b from a.
pub fn subtract(a: i64, b: i64) -> i64 {
    a - b
}

/// Multiplies two numbers.
pub fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

/// Divides a by b. Returns None if b is zero.
pub fn divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        None
    } else {
        Some(a / b)
    }
}

/// Calculates the average of a slice of numbers.
pub fn average(numbers: &[f64]) -> Option<f64> {
    if numbers.is_empty() {
        return None;
    }
    let sum: f64 = numbers.iter().sum();
    Some(sum / numbers.len() as f64)
}

/// Clamps a value between min and max.
pub fn clamp(value: i64, min: i64, max: i64) -> i64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

// Private helper — not visible outside this module
fn _gcd_internal(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

/// Greatest common divisor (public wrapper).
pub fn gcd(a: u64, b: u64) -> u64 {
    _gcd_internal(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_subtract() {
        assert_eq!(subtract(10, 3), 7);
    }

    #[test]
    fn test_multiply() {
        assert_eq!(multiply(4, 5), 20);
    }

    #[test]
    fn test_divide() {
        assert_eq!(divide(10.0, 2.0), Some(5.0));
        assert_eq!(divide(10.0, 0.0), None);
    }

    #[test]
    fn test_average() {
        assert_eq!(average(&[1.0, 2.0, 3.0]), Some(2.0));
        assert_eq!(average(&[]), None);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp(5, 0, 10), 5);
        assert_eq!(clamp(-5, 0, 10), 0);
        assert_eq!(clamp(15, 0, 10), 10);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 8), 4);
        assert_eq!(gcd(7, 3), 1);
    }
}
