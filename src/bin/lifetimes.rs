// Rust Tutorial #11: Lifetimes — How Rust Prevents Dangling References
// Demonstrates lifetime annotations, elision rules, lifetimes in structs,
// static lifetime, common patterns, and why the borrow checker needs them.

// --- Basic lifetime annotations ---

// The returned reference lives as long as the shortest input lifetime.
fn longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() >= s2.len() {
        s1
    } else {
        s2
    }
}

// Only one input has a lifetime tied to the output
fn first_word<'a>(text: &'a str) -> &'a str {
    match text.find(' ') {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// --- Lifetimes in structs ---

// A struct that holds a reference must have a lifetime annotation.
#[derive(Debug)]
struct Excerpt<'a> {
    text: &'a str,
}

impl<'a> Excerpt<'a> {
    // Method that returns a reference with the same lifetime
    fn first_sentence(&self) -> &str {
        match self.text.find('.') {
            Some(pos) => &self.text[..=pos],
            None => self.text,
        }
    }

    // Method that takes another reference with a different lifetime
    fn announce(&self, announcement: &str) -> String {
        format!("{}: {}", announcement, self.text)
    }
}

// Struct with multiple lifetime parameters
#[derive(Debug)]
struct Comparison<'a, 'b> {
    left: &'a str,
    right: &'b str,
}

impl<'a, 'b> Comparison<'a, 'b> {
    fn new(left: &'a str, right: &'b str) -> Comparison<'a, 'b> {
        Comparison { left, right }
    }

    fn both(&self) -> String {
        format!("{} vs {}", self.left, self.right)
    }
}

// --- Lifetime elision rules ---

// Rule 1: Each reference parameter gets its own lifetime.
// Rule 2: If there is exactly one input lifetime, it is assigned to all outputs.
// Rule 3: If &self or &mut self, the lifetime of self is assigned to outputs.

// These functions don't need explicit lifetimes because of elision rules:

// Rule 2 applies: one input, output gets same lifetime
fn trim_spaces(text: &str) -> &str {
    text.trim()
}

// Rule 3 applies: &self lifetime goes to output
struct TextProcessor {
    prefix: String,
}

impl TextProcessor {
    fn get_prefix(&self) -> &str {
        &self.prefix
    }

    fn process(&self, text: &str) -> String {
        format!("{}: {}", self.prefix, text)
    }
}

// --- Static lifetime ---

// 'static means the reference lives for the entire program.
fn get_greeting() -> &'static str {
    "Hello, world!" // String literals are always 'static
}

fn get_language() -> &'static str {
    "Rust"
}

// --- Common lifetime patterns ---

// Pattern: returning the longer of two strings
fn pick_longer<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}

// Pattern: finding something in a string
fn find_after<'a>(text: &'a str, marker: &str) -> &'a str {
    match text.find(marker) {
        Some(pos) => &text[pos + marker.len()..],
        None => "",
    }
}

// Pattern: split and return first part
fn before_colon<'a>(text: &'a str) -> &'a str {
    match text.find(':') {
        Some(pos) => &text[..pos],
        None => text,
    }
}

// --- Combining lifetimes with generics ---

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: std::fmt::Display,
{
    println!("Announcement: {}", ann);
    if x.len() >= y.len() { x } else { y }
}

fn main() {
    println!("=== Basic Lifetimes ===");
    let string1 = String::from("long string");
    let result;
    {
        let string2 = String::from("short");
        result = longer(&string1, &string2);
        println!("Longer: {}", result);
    }
    // result cannot be used here because string2 is dropped

    let word = first_word("hello world");
    println!("First word: {}", word);

    println!("\n=== Lifetimes in Structs ===");
    let text = String::from("Rust is fast. Rust is safe.");
    let excerpt = Excerpt { text: &text };
    println!("Excerpt: {:?}", excerpt);
    println!("First sentence: {}", excerpt.first_sentence());
    println!("Announced: {}", excerpt.announce("Important"));

    let comp = Comparison::new("Rust", "Go");
    println!("Comparison: {}", comp.both());

    println!("\n=== Lifetime Elision ===");
    let trimmed = trim_spaces("  hello  ");
    println!("Trimmed: '{}'", trimmed);

    let processor = TextProcessor {
        prefix: String::from("LOG"),
    };
    println!("Prefix: {}", processor.get_prefix());
    println!("Processed: {}", processor.process("something happened"));

    println!("\n=== Static Lifetime ===");
    println!("{}", get_greeting());
    println!("Language: {}", get_language());

    println!("\n=== Common Patterns ===");
    println!("Longer: {}", pick_longer("hello", "hi"));
    println!("After 'key=': {}", find_after("key=value", "key="));
    println!("Before colon: {}", before_colon("name:Alex"));

    println!("\n=== Lifetimes with Generics ===");
    let result = longest_with_announcement("hello", "hi", "Comparing strings");
    println!("Longest: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longer_first() {
        assert_eq!(longer("long string", "short"), "long string");
    }

    #[test]
    fn test_longer_second() {
        assert_eq!(longer("hi", "hello"), "hello");
    }

    #[test]
    fn test_longer_equal() {
        assert_eq!(longer("abc", "xyz"), "abc"); // first wins on tie
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
    }

    #[test]
    fn test_first_word_single() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn test_excerpt_first_sentence() {
        let text = String::from("Rust is fast. Rust is safe.");
        let excerpt = Excerpt { text: &text };
        assert_eq!(excerpt.first_sentence(), "Rust is fast.");
    }

    #[test]
    fn test_excerpt_no_period() {
        let text = String::from("No period here");
        let excerpt = Excerpt { text: &text };
        assert_eq!(excerpt.first_sentence(), "No period here");
    }

    #[test]
    fn test_excerpt_announce() {
        let text = String::from("Hello");
        let excerpt = Excerpt { text: &text };
        assert_eq!(excerpt.announce("Note"), "Note: Hello");
    }

    #[test]
    fn test_comparison() {
        let comp = Comparison::new("Rust", "Go");
        assert_eq!(comp.both(), "Rust vs Go");
    }

    #[test]
    fn test_trim_spaces() {
        assert_eq!(trim_spaces("  hello  "), "hello");
    }

    #[test]
    fn test_text_processor_prefix() {
        let processor = TextProcessor {
            prefix: String::from("LOG"),
        };
        assert_eq!(processor.get_prefix(), "LOG");
    }

    #[test]
    fn test_text_processor_process() {
        let processor = TextProcessor {
            prefix: String::from("LOG"),
        };
        assert_eq!(processor.process("event"), "LOG: event");
    }

    #[test]
    fn test_get_greeting() {
        assert_eq!(get_greeting(), "Hello, world!");
    }

    #[test]
    fn test_get_language() {
        assert_eq!(get_language(), "Rust");
    }

    #[test]
    fn test_pick_longer() {
        assert_eq!(pick_longer("hello", "hi"), "hello");
    }

    #[test]
    fn test_find_after() {
        assert_eq!(find_after("key=value", "key="), "value");
    }

    #[test]
    fn test_find_after_not_found() {
        assert_eq!(find_after("no marker", "xyz"), "");
    }

    #[test]
    fn test_before_colon() {
        assert_eq!(before_colon("name:Alex"), "name");
    }

    #[test]
    fn test_before_colon_none() {
        assert_eq!(before_colon("no colon"), "no colon");
    }

    #[test]
    fn test_longest_with_announcement() {
        let result = longest_with_announcement("hello", "hi", "test");
        assert_eq!(result, "hello");
    }
}
