// Rust Tutorial #5: Borrowing and References
// Demonstrates immutable references (&T), mutable references (&mut T),
// borrowing rules, slices, and dangling reference prevention.

fn main() {
    // --- Immutable References ---
    let name = String::from("Alex");
    let length = calculate_length(&name);
    println!("{} has {} characters", name, length);

    // Multiple immutable references are allowed
    let r1 = &name;
    let r2 = &name;
    println!("r1: {}, r2: {}", r1, r2);

    // --- Mutable References ---
    let mut greeting = String::from("Hello");
    add_suffix(&mut greeting, ", welcome!");
    println!("Greeting: {}", greeting);

    // --- Dereferencing ---
    let mut score = 50;
    double_in_place(&mut score);
    println!("Doubled score: {}", score);

    // --- String Slices ---
    let sentence = String::from("hello world");
    let word = first_word(&sentence);
    println!("First word: {}", word);

    // --- Borrowing with Functions ---
    let items = vec![
        String::from("Alex"),
        String::from("Jo"),
        String::from("Sam"),
        String::from("Jordan"),
    ];
    let long_names = filter_by_length(&items, 4);
    println!("Long names: {:?}", long_names);
    println!("Original items still valid: {:?}", items);

    // --- Mutable Vec Reference ---
    let mut numbers = vec![3, 1, 4, 1, 5, 9];
    sort_and_dedup(&mut numbers);
    println!("Sorted and deduped: {:?}", numbers);
}

/// Borrows a String immutably and returns its length.
fn calculate_length(s: &String) -> usize {
    s.len()
}

/// Borrows a String mutably and appends a suffix.
fn add_suffix(text: &mut String, suffix: &str) {
    text.push_str(suffix);
}

/// Borrows an i32 mutably and doubles it in place.
fn double_in_place(value: &mut i32) {
    *value *= 2;
}

/// Returns the first word from a string slice.
fn first_word(text: &str) -> &str {
    for (i, ch) in text.chars().enumerate() {
        if ch == ' ' {
            return &text[..i];
        }
    }
    text
}

/// Borrows a slice of Strings immutably, returns names with length >= min_length.
fn filter_by_length(names: &[String], min_length: usize) -> Vec<&String> {
    names.iter().filter(|n| n.len() >= min_length).collect()
}

/// Borrows a Vec mutably, sorts it and removes duplicates.
fn sort_and_dedup(numbers: &mut Vec<i32>) {
    numbers.sort();
    numbers.dedup();
}

/// Returns the longer of two string slices.
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

/// Counts how many items in a slice are above a threshold (borrows slice immutably).
fn count_matching(items: &[i32], threshold: i32) -> usize {
    items.iter().filter(|&&x| x > threshold).count()
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Immutable borrow — calculate_length works without taking ownership
    #[test]
    fn test_calculate_length() {
        let name = String::from("Alex");
        let len = calculate_length(&name);
        assert_eq!(len, 4);
        assert_eq!(name, "Alex");
    }

    // Test 2: Mutable borrow — add_suffix modifies the original String
    #[test]
    fn test_add_suffix() {
        let mut text = String::from("Hello");
        add_suffix(&mut text, " World");
        assert_eq!(text, "Hello World");
    }

    // Test 3: Mutable borrow with dereference — double_in_place
    #[test]
    fn test_double_in_place() {
        let mut value = 21;
        double_in_place(&mut value);
        assert_eq!(value, 42);
    }

    // Test 4: Double in place with zero
    #[test]
    fn test_double_in_place_zero() {
        let mut value = 0;
        double_in_place(&mut value);
        assert_eq!(value, 0);
    }

    // Test 5: String slice — first_word returns first word
    #[test]
    fn test_first_word_with_space() {
        let text = String::from("hello world");
        assert_eq!(first_word(&text), "hello");
    }

    // Test 6: String slice — first_word with no space returns entire string
    #[test]
    fn test_first_word_no_space() {
        let text = String::from("rust");
        assert_eq!(first_word(&text), "rust");
    }

    // Test 7: Filter by length — borrows immutably
    #[test]
    fn test_filter_by_length() {
        let names = vec![
            String::from("Alex"),
            String::from("Jo"),
            String::from("Sam"),
            String::from("Jordan"),
        ];
        let result = filter_by_length(&names, 4);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "Alex");
        assert_eq!(result[1], "Jordan");
        assert_eq!(names.len(), 4);
    }

    // Test 8: Sort and dedup — borrows mutably
    #[test]
    fn test_sort_and_dedup() {
        let mut numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
        sort_and_dedup(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 4, 5, 6, 9]);
    }

    // Test 9: Lifetime function — longer returns the longer string
    #[test]
    fn test_longer_first() {
        let result = longer("long string", "short");
        assert_eq!(result, "long string");
    }

    // Test 10: Lifetime function — longer with equal lengths returns first
    #[test]
    fn test_longer_equal() {
        let result = longer("abc", "xyz");
        assert_eq!(result, "abc");
    }

    // Test 11: Count matching — borrows slice immutably
    #[test]
    fn test_count_matching() {
        let data = vec![1, 5, 10, 15, 20];
        assert_eq!(count_matching(&data, 8), 3);
    }

    // Test 12: Count matching — nothing matches
    #[test]
    fn test_count_matching_none() {
        let data = vec![1, 2, 3];
        assert_eq!(count_matching(&data, 100), 0);
    }
}
