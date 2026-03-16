// Rust Tutorial #25: Macros — Writing Your Own
// Demonstrates macro_rules!, repetition patterns, macro hygiene,
// and practical macro examples.

use std::collections::HashMap;

// ========================================================
// Basic Macro — Simple Substitution
// ========================================================

macro_rules! say_hello {
    () => {
        println!("Hello from a macro!");
    };
}

// ========================================================
// Macro with Arguments
// ========================================================

macro_rules! greet {
    ($name:expr) => {
        format!("Hello, {}!", $name)
    };
}

macro_rules! add {
    ($a:expr, $b:expr) => {
        $a + $b
    };
}

// ========================================================
// Multiple Match Arms
// ========================================================

macro_rules! calculate {
    (add $a:expr, $b:expr) => {
        $a + $b
    };
    (sub $a:expr, $b:expr) => {
        $a - $b
    };
    (mul $a:expr, $b:expr) => {
        $a * $b
    };
    (div $a:expr, $b:expr) => {
        if $b != 0 {
            Some($a / $b)
        } else {
            None
        }
    };
}

// ========================================================
// Repetition — vec-like macro
// ========================================================

macro_rules! my_vec {
    () => {
        Vec::new()
    };
    ($($element:expr),+ $(,)?) => {
        {
            let mut v = Vec::new();
            $(v.push($element);)+
            v
        }
    };
}

// ========================================================
// HashMap macro
// ========================================================

macro_rules! hash_map {
    () => {
        HashMap::new()
    };
    ($($key:expr => $value:expr),+ $(,)?) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)+
            map
        }
    };
}

// ========================================================
// Assert-like Macros
// ========================================================

macro_rules! assert_between {
    ($value:expr, $min:expr, $max:expr) => {
        let val = $value;
        let min = $min;
        let max = $max;
        if val < min || val > max {
            panic!(
                "assertion failed: {} is not between {} and {} (value: {})",
                stringify!($value),
                min,
                max,
                val
            );
        }
    };
}

macro_rules! assert_contains {
    ($haystack:expr, $needle:expr) => {
        let haystack = &$haystack;
        let needle = &$needle;
        if !haystack.contains(needle) {
            panic!(
                "assertion failed: {:?} does not contain {:?}",
                haystack, needle
            );
        }
    };
}

// ========================================================
// Builder Macro — Generates a Builder Pattern
// ========================================================

macro_rules! builder {
    ($name:ident { $($field:ident : $type:ty),+ $(,)? }) => {
        #[derive(Debug, Clone)]
        struct $name {
            $($field: $type,)+
        }

        paste_builder!($name { $($field: $type),+ });
    };
}

macro_rules! paste_builder {
    ($name:ident { $($field:ident : $type:ty),+ }) => {
        impl $name {
            fn builder() -> concat_idents_builder!($name) {
                concat_idents_builder!($name) {
                    $($field: None,)+
                }
            }
        }
    };
}

// Since we can't use concat_idents in stable Rust, let's use a simpler builder approach:

macro_rules! make_struct {
    ($name:ident { $($field:ident : $type:ty),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        struct $name {
            $($field: $type,)+
        }

        impl $name {
            fn new($($field: $type),+) -> Self {
                Self { $($field,)+ }
            }
        }
    };
}

make_struct!(Config {
    host: String,
    port: u16,
    debug: bool,
});

make_struct!(Point {
    x: f64,
    y: f64,
});

// ========================================================
// Enum with Display Macro
// ========================================================

macro_rules! string_enum {
    ($name:ident { $($variant:ident => $display:expr),+ $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        enum $name {
            $($variant,)+
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant => write!(f, $display),)+
                }
            }
        }

        impl $name {
            fn all_variants() -> Vec<$name> {
                vec![$($name::$variant,)+]
            }
        }
    };
}

string_enum!(Color {
    Red => "red",
    Green => "green",
    Blue => "blue",
    Yellow => "yellow",
});

string_enum!(LogLevel {
    Debug => "DEBUG",
    Info => "INFO",
    Warn => "WARN",
    Error => "ERROR",
});

// ========================================================
// Retry Macro
// ========================================================

macro_rules! retry {
    ($attempts:expr, $body:expr) => {{
        let mut last_err = None;
        for attempt in 1..=$attempts {
            match $body {
                Ok(val) => return Ok(val),
                Err(e) => {
                    last_err = Some(format!("Attempt {}: {}", attempt, e));
                }
            }
        }
        Err(last_err.unwrap_or_else(|| "No attempts made".to_string()))
    }};
}

fn flaky_operation(counter: &std::cell::Cell<u32>) -> Result<String, String> {
    let count = counter.get();
    counter.set(count + 1);
    if count < 2 {
        Err(format!("Failed on attempt {}", count + 1))
    } else {
        Ok("Success!".to_string())
    }
}

fn run_with_retry() -> Result<String, String> {
    let counter = std::cell::Cell::new(0);
    retry!(5, flaky_operation(&counter))
}

// ========================================================
// Timing Macro
// ========================================================

macro_rules! time_it {
    ($label:expr, $body:expr) => {{
        let start = std::time::Instant::now();
        let result = $body;
        let elapsed = start.elapsed();
        println!("{}: {:?}", $label, elapsed);
        result
    }};
}

// ========================================================
// Log Macro with Levels
// ========================================================

macro_rules! log_msg {
    (debug, $($arg:tt)*) => {
        println!("[DEBUG] {}", format!($($arg)*));
    };
    (info, $($arg:tt)*) => {
        println!("[INFO] {}", format!($($arg)*));
    };
    (warn, $($arg:tt)*) => {
        println!("[WARN] {}", format!($($arg)*));
    };
    (error, $($arg:tt)*) => {
        println!("[ERROR] {}", format!($($arg)*));
    };
}

// ========================================================
// Stringify and Debugging
// ========================================================

macro_rules! debug_var {
    ($var:expr) => {
        println!("{} = {:?}", stringify!($var), $var);
    };
}

macro_rules! inspect {
    ($($var:expr),+) => {
        $(println!("  {} = {:?}", stringify!($var), $var);)+
    };
}

// ========================================================
// Main
// ========================================================

fn main() {
    println!("=== Macros Demo ===\n");

    // Demo 1: Basic macro
    println!("--- Basic Macro ---");
    say_hello!();
    let greeting = greet!("Alex");
    println!("{}", greeting);
    println!("2 + 3 = {}", add!(2, 3));

    println!();

    // Demo 2: Multiple arms
    println!("--- Calculate Macro ---");
    println!("add 5, 3 = {}", calculate!(add 5, 3));
    println!("sub 10, 4 = {}", calculate!(sub 10, 4));
    println!("mul 6, 7 = {}", calculate!(mul 6, 7));
    println!("div 10, 3 = {:?}", calculate!(div 10, 3));
    println!("div 10, 0 = {:?}", calculate!(div 10, 0));

    println!();

    // Demo 3: my_vec
    println!("--- my_vec Macro ---");
    let empty: Vec<i32> = my_vec![];
    let nums = my_vec![1, 2, 3, 4, 5];
    let with_trailing = my_vec![10, 20, 30,];
    println!("Empty: {:?}", empty);
    println!("Nums: {:?}", nums);
    println!("Trailing comma: {:?}", with_trailing);

    println!();

    // Demo 4: hash_map
    println!("--- hash_map Macro ---");
    let empty_map: HashMap<String, i32> = hash_map![];
    let scores = hash_map![
        "Alex" => 95,
        "Sam" => 87,
        "Jordan" => 92,
    ];
    println!("Empty: {:?}", empty_map);
    println!("Scores: {:?}", scores);

    println!();

    // Demo 5: assert_between
    println!("--- Assert Macros ---");
    let age = 25;
    assert_between!(age, 18, 65);
    println!("Age {} is between 18 and 65", age);

    let text = "Hello, World!";
    assert_contains!(text, "World");
    println!("Text contains 'World'");

    println!();

    // Demo 6: make_struct
    println!("--- make_struct Macro ---");
    let config = Config::new("localhost".to_string(), 8080, true);
    println!("Config: {:?}", config);

    let point = Point::new(3.0, 4.0);
    println!("Point: {:?}", point);

    println!();

    // Demo 7: string_enum
    println!("--- string_enum Macro ---");
    let color = Color::Red;
    println!("Color: {}", color);
    println!("All colors: {:?}", Color::all_variants().iter().map(|c| c.to_string()).collect::<Vec<_>>());

    let level = LogLevel::Warn;
    println!("Level: {}", level);
    println!("All levels: {:?}", LogLevel::all_variants().iter().map(|l| l.to_string()).collect::<Vec<_>>());

    println!();

    // Demo 8: retry
    println!("--- Retry Macro ---");
    match run_with_retry() {
        Ok(msg) => println!("Retry succeeded: {}", msg),
        Err(e) => println!("Retry failed: {}", e),
    }

    println!();

    // Demo 9: time_it
    println!("--- Timing Macro ---");
    let sum = time_it!("Sum calculation", {
        (0..1_000_000).sum::<u64>()
    });
    println!("Sum: {}", sum);

    println!();

    // Demo 10: log_msg
    println!("--- Log Macro ---");
    log_msg!(debug, "Starting application");
    log_msg!(info, "Server listening on port {}", 8080);
    log_msg!(warn, "Memory usage at {}%", 85);
    log_msg!(error, "Connection failed: {}", "timeout");

    println!();

    // Demo 11: debug_var and inspect
    println!("--- Debug Macros ---");
    let name = "Alex";
    let score = 95;
    let passed = true;
    debug_var!(name);
    debug_var!(score);
    debug_var!(2 + 2);

    println!("Inspection:");
    inspect!(name, score, passed);

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet_macro() {
        assert_eq!(greet!("Sam"), "Hello, Sam!");
        assert_eq!(greet!("Alex"), "Hello, Alex!");
    }

    #[test]
    fn test_add_macro() {
        assert_eq!(add!(2, 3), 5);
        assert_eq!(add!(10, 20), 30);
        assert_eq!(add!(1.5, 2.5), 4.0);
    }

    #[test]
    fn test_calculate_add() {
        assert_eq!(calculate!(add 5, 3), 8);
    }

    #[test]
    fn test_calculate_sub() {
        assert_eq!(calculate!(sub 10, 4), 6);
    }

    #[test]
    fn test_calculate_mul() {
        assert_eq!(calculate!(mul 6, 7), 42);
    }

    #[test]
    fn test_calculate_div() {
        assert_eq!(calculate!(div 10, 2), Some(5));
    }

    #[test]
    fn test_calculate_div_by_zero() {
        assert_eq!(calculate!(div 10, 0), None);
    }

    #[test]
    fn test_my_vec_empty() {
        let v: Vec<i32> = my_vec![];
        assert!(v.is_empty());
    }

    #[test]
    fn test_my_vec_elements() {
        let v = my_vec![1, 2, 3];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_my_vec_trailing_comma() {
        let v = my_vec![1, 2, 3,];
        assert_eq!(v, vec![1, 2, 3]);
    }

    #[test]
    fn test_my_vec_single() {
        let v = my_vec![42];
        assert_eq!(v, vec![42]);
    }

    #[test]
    fn test_my_vec_strings() {
        let v = my_vec!["a".to_string(), "b".to_string()];
        assert_eq!(v, vec!["a".to_string(), "b".to_string()]);
    }

    #[test]
    fn test_hash_map_empty() {
        let m: HashMap<String, i32> = hash_map![];
        assert!(m.is_empty());
    }

    #[test]
    fn test_hash_map_entries() {
        let m = hash_map!["a" => 1, "b" => 2, "c" => 3];
        assert_eq!(m.len(), 3);
        assert_eq!(m["a"], 1);
        assert_eq!(m["b"], 2);
        assert_eq!(m["c"], 3);
    }

    #[test]
    fn test_hash_map_trailing_comma() {
        let m = hash_map!["x" => 10,];
        assert_eq!(m.len(), 1);
        assert_eq!(m["x"], 10);
    }

    #[test]
    fn test_assert_between_pass() {
        assert_between!(5, 1, 10);
        assert_between!(1, 1, 10);
        assert_between!(10, 1, 10);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_between_fail_low() {
        assert_between!(0, 1, 10);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn test_assert_between_fail_high() {
        assert_between!(11, 1, 10);
    }

    #[test]
    fn test_assert_contains_pass() {
        assert_contains!("hello world", "world");
    }

    #[test]
    #[should_panic(expected = "does not contain")]
    fn test_assert_contains_fail() {
        assert_contains!("hello world", "xyz");
    }

    #[test]
    fn test_make_struct_config() {
        let config = Config::new("localhost".to_string(), 8080, true);
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 8080);
        assert!(config.debug);
    }

    #[test]
    fn test_make_struct_point() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }

    #[test]
    fn test_make_struct_equality() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(1.0, 2.0);
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_string_enum_display() {
        assert_eq!(Color::Red.to_string(), "red");
        assert_eq!(Color::Green.to_string(), "green");
        assert_eq!(Color::Blue.to_string(), "blue");
        assert_eq!(Color::Yellow.to_string(), "yellow");
    }

    #[test]
    fn test_string_enum_all_variants() {
        let colors = Color::all_variants();
        assert_eq!(colors.len(), 4);
        assert_eq!(colors[0], Color::Red);
        assert_eq!(colors[3], Color::Yellow);
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(LogLevel::Debug.to_string(), "DEBUG");
        assert_eq!(LogLevel::Info.to_string(), "INFO");
        assert_eq!(LogLevel::Warn.to_string(), "WARN");
        assert_eq!(LogLevel::Error.to_string(), "ERROR");
    }

    #[test]
    fn test_log_level_all_variants() {
        let levels = LogLevel::all_variants();
        assert_eq!(levels.len(), 4);
    }

    #[test]
    fn test_retry_succeeds() {
        let result = run_with_retry();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Success!");
    }

    #[test]
    fn test_flaky_operation_fails_first() {
        let counter = std::cell::Cell::new(0);
        let result = flaky_operation(&counter);
        assert!(result.is_err());
    }

    #[test]
    fn test_flaky_operation_succeeds_on_third() {
        let counter = std::cell::Cell::new(2);
        let result = flaky_operation(&counter);
        assert!(result.is_ok());
    }
}
