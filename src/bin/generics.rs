// Rust Tutorial #10: Generics — Write Code That Works with Any Type
// Demonstrates generic functions, generic structs, generic enums,
// trait bounds, where clauses, multiple bounds, monomorphization, and impl blocks.

use std::fmt;

// --- Generic functions ---

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut biggest = &list[0];
    for item in &list[1..] {
        if item > biggest {
            biggest = item;
        }
    }
    biggest
}

fn first_or_default<T: Default + Clone>(list: &[T]) -> T {
    match list.first() {
        Some(item) => item.clone(),
        None => T::default(),
    }
}

// --- Generic structs ---

#[derive(Debug, PartialEq)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }

    fn swap(self) -> Pair<T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

// Methods only available when T has certain traits
impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn larger(&self) -> &T {
        if self.first >= self.second {
            &self.first
        } else {
            &self.second
        }
    }

    fn display_larger(&self) -> String {
        format!("The larger value is: {}", self.larger())
    }
}

// Struct with two different generic types
#[derive(Debug, PartialEq)]
struct KeyValue<K, V> {
    key: K,
    value: V,
}

impl<K: fmt::Display, V: fmt::Display> KeyValue<K, V> {
    fn new(key: K, value: V) -> KeyValue<K, V> {
        KeyValue { key, value }
    }

    fn format_entry(&self) -> String {
        format!("{}: {}", self.key, self.value)
    }
}

// --- Generic enums ---

#[derive(Debug, PartialEq)]
enum Container<T> {
    Empty,
    Single(T),
    Multiple(Vec<T>),
}

impl<T: Clone> Container<T> {
    fn count(&self) -> usize {
        match self {
            Container::Empty => 0,
            Container::Single(_) => 1,
            Container::Multiple(items) => items.len(),
        }
    }

    fn first(&self) -> Option<&T> {
        match self {
            Container::Empty => None,
            Container::Single(item) => Some(item),
            Container::Multiple(items) => items.first(),
        }
    }
}

// --- Where clauses ---

fn print_pair<T>(pair: &Pair<T>)
where
    T: fmt::Display + fmt::Debug,
{
    println!("Display: {} and {}", pair.first, pair.second);
    println!("Debug: {:?} and {:?}", pair.first, pair.second);
}

// Multiple bounds with where clause
fn compare_and_display<T, U>(t: &T, u: &U) -> String
where
    T: fmt::Display + PartialOrd,
    U: fmt::Display + Clone,
{
    format!("T={}, U={}", t, u)
}

// --- Generic function that returns a generic type ---

fn wrap_in_vec<T>(item: T) -> Vec<T> {
    vec![item]
}

fn repeat<T: Clone>(item: &T, count: usize) -> Vec<T> {
    let mut result = Vec::with_capacity(count);
    for _ in 0..count {
        result.push(item.clone());
    }
    result
}

// --- A practical example: generic Stack ---

#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    fn size(&self) -> usize {
        self.items.len()
    }
}

fn main() {
    println!("=== Generic Functions ===");
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest number: {}", largest(&numbers));

    let words = vec!["apple", "banana", "cherry"];
    println!("Largest word: {}", largest(&words));

    let empty_ints: Vec<i32> = vec![];
    println!("Default for empty: {}", first_or_default(&empty_ints));

    println!("\n=== Generic Structs ===");
    let pair = Pair::new(10, 20);
    println!("Pair: {:?}", pair);
    println!("Larger: {}", pair.display_larger());

    let swapped = Pair::new(1, 2).swap();
    println!("Swapped: {:?}", swapped);

    let kv = KeyValue::new("name", "Alex");
    println!("Entry: {}", kv.format_entry());

    println!("\n=== Generic Enums ===");
    let empty: Container<i32> = Container::Empty;
    let single = Container::Single(42);
    let multi = Container::Multiple(vec![1, 2, 3]);
    println!("Empty count: {}", empty.count());
    println!("Single first: {:?}", single.first());
    println!("Multi count: {}", multi.count());

    println!("\n=== Where Clauses ===");
    let str_pair = Pair::new(String::from("hello"), String::from("world"));
    print_pair(&str_pair);
    let result = compare_and_display(&42, &"hello");
    println!("{}", result);

    println!("\n=== Generic Utilities ===");
    let wrapped = wrap_in_vec(42);
    println!("Wrapped: {:?}", wrapped);
    let repeated = repeat(&"hi", 3);
    println!("Repeated: {:?}", repeated);

    println!("\n=== Generic Stack ===");
    let mut stack: Stack<i32> = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);
    println!("Top: {:?}", stack.peek());
    println!("Pop: {:?}", stack.pop());
    println!("Size: {}", stack.size());
    println!("Empty: {}", stack.is_empty());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_numbers() {
        let numbers = vec![34, 50, 25, 100, 65];
        assert_eq!(*largest(&numbers), 100);
    }

    #[test]
    fn test_largest_strings() {
        let words = vec!["apple", "banana", "cherry"];
        assert_eq!(*largest(&words), "cherry");
    }

    #[test]
    fn test_first_or_default_with_values() {
        let numbers = vec![10, 20, 30];
        assert_eq!(first_or_default(&numbers), 10);
    }

    #[test]
    fn test_first_or_default_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(first_or_default(&empty), 0);
    }

    #[test]
    fn test_pair_new() {
        let pair = Pair::new(1, 2);
        assert_eq!(pair.first, 1);
        assert_eq!(pair.second, 2);
    }

    #[test]
    fn test_pair_swap() {
        let pair = Pair::new(1, 2).swap();
        assert_eq!(pair.first, 2);
        assert_eq!(pair.second, 1);
    }

    #[test]
    fn test_pair_larger() {
        let pair = Pair::new(10, 20);
        assert_eq!(*pair.larger(), 20);
    }

    #[test]
    fn test_pair_display_larger() {
        let pair = Pair::new(5, 15);
        assert_eq!(pair.display_larger(), "The larger value is: 15");
    }

    #[test]
    fn test_key_value() {
        let kv = KeyValue::new("name", "Alex");
        assert_eq!(kv.format_entry(), "name: Alex");
    }

    #[test]
    fn test_container_empty() {
        let c: Container<i32> = Container::Empty;
        assert_eq!(c.count(), 0);
        assert_eq!(c.first(), None);
    }

    #[test]
    fn test_container_single() {
        let c = Container::Single(42);
        assert_eq!(c.count(), 1);
        assert_eq!(c.first(), Some(&42));
    }

    #[test]
    fn test_container_multiple() {
        let c = Container::Multiple(vec![1, 2, 3]);
        assert_eq!(c.count(), 3);
        assert_eq!(c.first(), Some(&1));
    }

    #[test]
    fn test_wrap_in_vec() {
        let result = wrap_in_vec(42);
        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_repeat() {
        let result = repeat(&"hi", 3);
        assert_eq!(result, vec!["hi", "hi", "hi"]);
    }

    #[test]
    fn test_repeat_zero() {
        let result = repeat(&42, 0);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_stack_push_pop() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }

    #[test]
    fn test_stack_peek() {
        let mut stack = Stack::new();
        stack.push(10);
        assert_eq!(stack.peek(), Some(&10));
        assert_eq!(stack.size(), 1); // peek doesn't remove
    }

    #[test]
    fn test_stack_is_empty() {
        let mut stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        stack.push(1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_compare_and_display() {
        let result = compare_and_display(&42, &"hello");
        assert_eq!(result, "T=42, U=hello");
    }
}
