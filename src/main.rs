// Rust Tutorial #4: Ownership — The Key Concept
// Demonstrates move semantics, Copy trait, Clone, scope/drop,
// and how ownership works with functions.

fn main() {
    // --- Move Semantics ---
    // String is heap-allocated, so assignment moves ownership
    let name = String::from("Alex");
    let other_name = name; // ownership moves to other_name
    // name is no longer valid here
    println!("Moved name: {}", other_name);

    // --- Copy Trait ---
    // Integers live on the stack and implement Copy
    let age = 25;
    let other_age = age; // copy, not move
    println!("Original age: {}, copied age: {}", age, other_age);

    // --- Clone ---
    // Explicit deep copy of heap data
    let greeting = String::from("Hello");
    let greeting_clone = greeting.clone();
    println!("Original: {}, Clone: {}", greeting, greeting_clone);

    // --- Scope and Drop ---
    // Values are dropped when they go out of scope
    {
        let scoped = String::from("I exist only in this block");
        println!("{}", scoped);
    } // scoped is dropped here

    // --- Ownership and Functions ---
    // Passing a String to a function moves ownership
    let message = String::from("Good morning");
    print_message(message);
    // message is no longer valid here

    // Copy types are copied when passed to functions
    let score = 42;
    print_score(score);
    println!("Score after function call: {}", score); // still valid

    // --- Return Values Transfer Ownership ---
    let result = create_greeting("Sam");
    println!("{}", result);

    // Take and give back ownership
    let text = String::from("Hello");
    let text = add_exclamation(text);
    println!("With exclamation: {}", text);

    // --- Tuple of Copy types copies ---
    let point = (10, 20);
    let other_point = point;
    println!("point: {:?}, other_point: {:?}", point, other_point);

    // --- Multiple moves in sequence ---
    let data = String::from("ownership chain");
    let second = data;
    let third = second;
    // only third is valid now
    println!("Final owner: {}", third);
}

/// Prints a message. Takes ownership of the String.
fn print_message(msg: String) {
    println!("Message: {}", msg);
    // msg is dropped here
}

/// Prints a score. i32 implements Copy, so the caller keeps its value.
fn print_score(s: i32) {
    println!("Score: {}", s);
}

/// Creates a greeting String and returns ownership to the caller.
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)
}

/// Takes ownership of a String, modifies it, and returns ownership.
fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: String move makes original invalid (we verify the new owner works)
    #[test]
    fn test_string_move() {
        let original = String::from("Alex");
        let moved = original;
        assert_eq!(moved, "Alex");
    }

    // Test 2: i32 copies — both variables remain valid
    #[test]
    fn test_i32_copy() {
        let a = 42;
        let b = a;
        assert_eq!(a, 42);
        assert_eq!(b, 42);
    }

    // Test 3: f64 copies — both variables remain valid
    #[test]
    fn test_f64_copy() {
        let x: f64 = 3.14;
        let y = x;
        assert!((x - 3.14_f64).abs() < f64::EPSILON);
        assert!((y - 3.14_f64).abs() < f64::EPSILON);
    }

    // Test 4: bool copies
    #[test]
    fn test_bool_copy() {
        let flag = true;
        let other = flag;
        assert!(flag);
        assert!(other);
    }

    // Test 5: Clone creates an independent deep copy
    #[test]
    fn test_clone_independence() {
        let mut original = String::from("Hello");
        let cloned = original.clone();
        original.push_str(" World");
        assert_eq!(original, "Hello World");
        assert_eq!(cloned, "Hello"); // clone is unaffected
    }

    // Test 6: Function takes ownership and uses the value
    #[test]
    fn test_function_takes_ownership() {
        fn consume(s: String) -> usize {
            s.len()
        }
        let data = String::from("Rust");
        let length = consume(data);
        assert_eq!(length, 4);
    }

    // Test 7: Function returns ownership
    #[test]
    fn test_create_greeting() {
        let greeting = create_greeting("Jordan");
        assert_eq!(greeting, "Hello, Jordan!");
    }

    // Test 8: Take and return ownership with modification
    #[test]
    fn test_add_exclamation() {
        let text = String::from("Hi");
        let result = add_exclamation(text);
        assert_eq!(result, "Hi!");
    }

    // Test 9: Tuple of Copy types copies correctly
    #[test]
    fn test_tuple_copy() {
        let pair = (10, 20);
        let other = pair;
        assert_eq!(pair.0, other.0);
        assert_eq!(pair.1, other.1);
    }

    // Test 10: Ownership chain — only the last owner has the value
    #[test]
    fn test_ownership_chain() {
        let first = String::from("data");
        let second = first;
        let third = second;
        assert_eq!(third, "data");
    }

    // Test 11: Vec moves on assignment (heap-allocated)
    #[test]
    fn test_vec_move() {
        let v1 = vec![1, 2, 3];
        let v2 = v1;
        assert_eq!(v2, vec![1, 2, 3]);
    }

    // Test 12: Clone of Vec creates independent copy
    #[test]
    fn test_vec_clone() {
        let mut v1 = vec![1, 2, 3];
        let v2 = v1.clone();
        v1.push(4);
        assert_eq!(v1, vec![1, 2, 3, 4]);
        assert_eq!(v2, vec![1, 2, 3]); // clone unaffected
    }

    // Test 13: Scope drop — variable created in inner scope doesn't leak
    #[test]
    fn test_scope_drop() {
        let outside = String::from("outside");
        {
            let _inside = String::from("inside");
            // _inside is dropped at end of this block
        }
        assert_eq!(outside, "outside"); // outside still valid
    }

    // Test 14: Copy type passed to function — caller retains value
    #[test]
    fn test_copy_passed_to_function() {
        fn double(x: i32) -> i32 {
            x * 2
        }
        let num = 5;
        let result = double(num);
        assert_eq!(num, 5); // still valid after function call
        assert_eq!(result, 10);
    }
}
