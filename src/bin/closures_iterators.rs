// Rust Tutorial #12: Closures and Iterators
// Demonstrates closure syntax, Fn/FnMut/FnOnce traits, move closures,
// iterator trait, iter/into_iter/iter_mut, map/filter/fold/collect,
// chaining, and implementing Iterator for custom types.

// --- Closure basics ---

fn apply_operation(a: i32, b: i32, op: impl Fn(i32, i32) -> i32) -> i32 {
    op(a, b)
}

fn apply_to_vec(numbers: &[i32], op: impl Fn(i32) -> i32) -> Vec<i32> {
    numbers.iter().map(|&n| op(n)).collect()
}

// --- FnMut: closures that modify captured variables ---

fn count_matches(items: &[&str], predicate: impl Fn(&str) -> bool) -> usize {
    let mut count = 0;
    for item in items {
        if predicate(item) {
            count += 1;
        }
    }
    count
}

fn apply_mut<F>(times: usize, mut f: F)
where
    F: FnMut(usize),
{
    for i in 0..times {
        f(i);
    }
}

// --- FnOnce: closures that consume captured values ---

fn consume_and_print<F>(f: F)
where
    F: FnOnce() -> String,
{
    println!("{}", f());
}

// --- Returning closures ---

fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// --- Custom iterator ---

struct Counter {
    current: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { current: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}

// --- Fibonacci iterator ---

struct Fibonacci {
    a: u64,
    b: u64,
    count: usize,
    max: usize,
}

impl Fibonacci {
    fn new(max: usize) -> Fibonacci {
        Fibonacci {
            a: 0,
            b: 1,
            count: 0,
            max,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.count >= self.max {
            return None;
        }
        let result = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        self.count += 1;
        Some(result)
    }
}

// --- Practical iterator examples ---

fn sum_of_squares(numbers: &[i32]) -> i32 {
    numbers.iter().map(|&x| x * x).sum()
}

fn even_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().filter(|&&x| x % 2 == 0).copied().collect()
}

fn word_lengths(words: &[&str]) -> Vec<usize> {
    words.iter().map(|w| w.len()).collect()
}

fn flatten_and_sort(nested: &[Vec<i32>]) -> Vec<i32> {
    let mut result: Vec<i32> = nested.iter().flatten().copied().collect();
    result.sort();
    result
}

fn join_with(items: &[&str], separator: &str) -> String {
    items
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(separator)
}

fn main() {
    println!("=== Closure Basics ===");
    let add = |a, b| a + b;
    let multiply = |a, b| a * b;
    println!("Add: {}", apply_operation(3, 4, add));
    println!("Multiply: {}", apply_operation(3, 4, multiply));

    let doubled = apply_to_vec(&[1, 2, 3, 4], |x| x * 2);
    println!("Doubled: {:?}", doubled);

    println!("\n=== FnMut Closures ===");
    let mut results = Vec::new();
    apply_mut(5, |i| {
        results.push(i * i);
    });
    println!("Squares: {:?}", results);

    println!("\n=== Move Closures ===");
    let name = String::from("Alex");
    consume_and_print(move || {
        format!("Hello, {}!", name)
    });
    // name is moved — cannot use it here

    println!("\n=== Returning Closures ===");
    let add_five = make_adder(5);
    let double = make_multiplier(2);
    println!("10 + 5 = {}", add_five(10));
    println!("10 * 2 = {}", double(10));

    println!("\n=== Iterator Methods ===");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // map
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    println!("Squares: {:?}", squares);

    // filter
    let evens: Vec<&i32> = numbers.iter().filter(|&&x| x % 2 == 0).collect();
    println!("Evens: {:?}", evens);

    // fold
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // chaining
    let result: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();
    println!("Sum of even squares: {}", result);

    println!("\n=== iter vs into_iter vs iter_mut ===");
    let words = vec![String::from("hello"), String::from("world")];

    // iter() — borrows, original vec still usable
    for word in words.iter() {
        println!("Borrowed: {}", word);
    }
    println!("Vec still here: {:?}", words);

    // iter_mut() — mutable borrow
    let mut scores = vec![80, 90, 70];
    for score in scores.iter_mut() {
        *score += 10; // Add bonus
    }
    println!("With bonus: {:?}", scores);

    // into_iter() — consumes the vec
    let items = vec![1, 2, 3];
    let doubled: Vec<i32> = items.into_iter().map(|x| x * 2).collect();
    println!("Consumed and doubled: {:?}", doubled);
    // items is moved — cannot use it here

    println!("\n=== Custom Iterator ===");
    let counter = Counter::new(5);
    let values: Vec<u32> = counter.collect();
    println!("Counter: {:?}", values);

    println!("\n=== Fibonacci Iterator ===");
    let fib: Vec<u64> = Fibonacci::new(10).collect();
    println!("Fibonacci: {:?}", fib);

    println!("\n=== Practical Examples ===");
    println!("Sum of squares: {}", sum_of_squares(&[1, 2, 3, 4]));
    println!("Even numbers: {:?}", even_numbers(&[1, 2, 3, 4, 5, 6]));
    println!("Word lengths: {:?}", word_lengths(&["hello", "world", "hi"]));

    let nested = vec![vec![3, 1], vec![4, 2]];
    println!("Flattened and sorted: {:?}", flatten_and_sort(&nested));
    println!("Joined: {}", join_with(&["a", "b", "c"], ", "));

    // Count matches with closure
    let fruits = vec!["apple", "avocado", "banana", "apricot"];
    let a_count = count_matches(&fruits, |s| s.starts_with('a'));
    println!("Fruits starting with 'a': {}", a_count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_operation_add() {
        assert_eq!(apply_operation(3, 4, |a, b| a + b), 7);
    }

    #[test]
    fn test_apply_operation_multiply() {
        assert_eq!(apply_operation(3, 4, |a, b| a * b), 12);
    }

    #[test]
    fn test_apply_to_vec() {
        let result = apply_to_vec(&[1, 2, 3], |x| x * 2);
        assert_eq!(result, vec![2, 4, 6]);
    }

    #[test]
    fn test_count_matches() {
        let fruits = vec!["apple", "avocado", "banana", "apricot"];
        assert_eq!(count_matches(&fruits, |s| s.starts_with('a')), 3);
    }

    #[test]
    fn test_count_matches_none() {
        let fruits = vec!["apple", "banana"];
        assert_eq!(count_matches(&fruits, |s| s.starts_with('z')), 0);
    }

    #[test]
    fn test_apply_mut() {
        let mut results = Vec::new();
        apply_mut(4, |i| results.push(i));
        assert_eq!(results, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_make_adder() {
        let add_five = make_adder(5);
        assert_eq!(add_five(10), 15);
        assert_eq!(add_five(0), 5);
    }

    #[test]
    fn test_make_multiplier() {
        let double = make_multiplier(2);
        assert_eq!(double(5), 10);
        assert_eq!(double(0), 0);
    }

    #[test]
    fn test_counter() {
        let values: Vec<u32> = Counter::new(5).collect();
        assert_eq!(values, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_counter_zero() {
        let values: Vec<u32> = Counter::new(0).collect();
        assert_eq!(values, Vec::<u32>::new());
    }

    #[test]
    fn test_counter_sum() {
        let sum: u32 = Counter::new(5).sum();
        assert_eq!(sum, 15);
    }

    #[test]
    fn test_fibonacci() {
        let fib: Vec<u64> = Fibonacci::new(8).collect();
        assert_eq!(fib, vec![0, 1, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn test_fibonacci_zero() {
        let fib: Vec<u64> = Fibonacci::new(0).collect();
        assert_eq!(fib, Vec::<u64>::new());
    }

    #[test]
    fn test_sum_of_squares() {
        assert_eq!(sum_of_squares(&[1, 2, 3]), 14);
    }

    #[test]
    fn test_sum_of_squares_empty() {
        assert_eq!(sum_of_squares(&[]), 0);
    }

    #[test]
    fn test_even_numbers() {
        assert_eq!(even_numbers(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }

    #[test]
    fn test_even_numbers_none() {
        assert_eq!(even_numbers(&[1, 3, 5]), Vec::<i32>::new());
    }

    #[test]
    fn test_word_lengths() {
        assert_eq!(word_lengths(&["hi", "hello", "hey"]), vec![2, 5, 3]);
    }

    #[test]
    fn test_flatten_and_sort() {
        let nested = vec![vec![3, 1], vec![4, 2]];
        assert_eq!(flatten_and_sort(&nested), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_join_with() {
        assert_eq!(join_with(&["a", "b", "c"], ", "), "a, b, c");
    }

    #[test]
    fn test_join_with_single() {
        assert_eq!(join_with(&["hello"], "-"), "hello");
    }

    #[test]
    fn test_chaining() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let result: i32 = numbers
            .iter()
            .filter(|&&x| x % 2 == 0)
            .map(|&x| x * x)
            .sum();
        assert_eq!(result, 56); // 4 + 16 + 36
    }

    #[test]
    fn test_counter_with_map() {
        let doubled: Vec<u32> = Counter::new(3).map(|x| x * 2).collect();
        assert_eq!(doubled, vec![2, 4, 6]);
    }
}
