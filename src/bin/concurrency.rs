// Rust Tutorial #14: Concurrency — Threads, Channels, and Mutex
// Demonstrates thread::spawn, move closures with threads, message passing
// with mpsc channels, shared state with Mutex<T>, Arc<Mutex<T>> pattern,
// Send/Sync traits, and fearless concurrency.

use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// --- Basic thread spawning ---

fn spawn_basic_thread() -> String {
    let handle = thread::spawn(|| {
        String::from("Hello from thread!")
    });
    handle.join().unwrap()
}

// --- Move closures with threads ---

fn spawn_with_data(data: Vec<i32>) -> i32 {
    let handle = thread::spawn(move || {
        data.iter().sum::<i32>()
    });
    handle.join().unwrap()
}

fn parallel_sum(numbers: Vec<i32>, chunk_size: usize) -> i32 {
    let chunks: Vec<Vec<i32>> = numbers
        .chunks(chunk_size)
        .map(|c| c.to_vec())
        .collect();

    let handles: Vec<thread::JoinHandle<i32>> = chunks
        .into_iter()
        .map(|chunk| {
            thread::spawn(move || chunk.iter().sum())
        })
        .collect();

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

// --- Message passing with channels ---

fn channel_basic() -> Vec<String> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("hello"),
            String::from("from"),
            String::from("thread"),
        ];
        for msg in messages {
            tx.send(msg).unwrap();
        }
    });

    let mut received = Vec::new();
    for msg in rx {
        received.push(msg);
    }
    received
}

fn channel_multiple_producers() -> Vec<i32> {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(i * 10).unwrap();
        });
    }
    drop(tx); // Drop original so rx iterator ends

    let mut results: Vec<i32> = rx.iter().collect();
    results.sort();
    results
}

// --- Pipeline with channels ---

fn pipeline_transform(input: Vec<i32>) -> Vec<i32> {
    // Stage 1: double
    let (tx1, rx1) = mpsc::channel();
    thread::spawn(move || {
        for value in input {
            tx1.send(value * 2).unwrap();
        }
    });

    // Stage 2: add 1
    let (tx2, rx2) = mpsc::channel();
    thread::spawn(move || {
        for value in rx1 {
            tx2.send(value + 1).unwrap();
        }
    });

    rx2.iter().collect()
}

// --- Mutex<T>: shared mutable state ---

fn mutex_basic() -> i32 {
    let counter = Mutex::new(0);
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }
    {
        let mut num = counter.lock().unwrap();
        *num += 1;
    }
    *counter.lock().unwrap()
}

// --- Arc<Mutex<T>>: thread-safe shared mutable state ---

fn arc_mutex_counter(thread_count: usize, increments_per_thread: usize) -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();

    for _ in 0..thread_count {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments_per_thread {
                let mut num = counter_clone.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *counter.lock().unwrap()
}

fn arc_mutex_shared_vec(thread_count: usize) -> Vec<String> {
    let data = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();

    for i in 0..thread_count {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data_clone.lock().unwrap();
            vec.push(format!("thread-{}", i));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let mut result = data.lock().unwrap().clone();
    result.sort();
    result
}

// --- Practical example: parallel word count ---

fn parallel_word_count(texts: Vec<String>) -> usize {
    let total = Arc::new(Mutex::new(0usize));
    let mut handles = Vec::new();

    for text in texts {
        let total_clone = Arc::clone(&total);
        let handle = thread::spawn(move || {
            let count = text.split_whitespace().count();
            let mut total = total_clone.lock().unwrap();
            *total += count;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    *total.lock().unwrap()
}

// --- Practical example: parallel map ---

fn parallel_map<T, U, F>(items: Vec<T>, f: F) -> Vec<U>
where
    T: Send + 'static,
    U: Send + 'static,
    F: Fn(T) -> U + Send + Sync + 'static,
{
    let f = Arc::new(f);
    let handles: Vec<(usize, thread::JoinHandle<U>)> = items
        .into_iter()
        .enumerate()
        .map(|(i, item)| {
            let f = Arc::clone(&f);
            (i, thread::spawn(move || f(item)))
        })
        .collect();

    let mut results: Vec<(usize, U)> = handles
        .into_iter()
        .map(|(i, h)| (i, h.join().unwrap()))
        .collect();

    results.sort_by_key(|(i, _)| *i);
    results.into_iter().map(|(_, v)| v).collect()
}

fn main() {
    println!("=== Basic Threads ===");
    let msg = spawn_basic_thread();
    println!("{}", msg);

    println!("\n=== Move Closures ===");
    let data = vec![1, 2, 3, 4, 5];
    let sum = spawn_with_data(data);
    println!("Sum from thread: {}", sum);

    println!("\n=== Parallel Sum ===");
    let numbers: Vec<i32> = (1..=100).collect();
    let total = parallel_sum(numbers, 25);
    println!("Parallel sum of 1..100: {}", total);

    println!("\n=== Channel: Basic ===");
    let messages = channel_basic();
    for msg in &messages {
        println!("Received: {}", msg);
    }

    println!("\n=== Channel: Multiple Producers ===");
    let results = channel_multiple_producers();
    println!("From multiple producers: {:?}", results);

    println!("\n=== Channel: Pipeline ===");
    let transformed = pipeline_transform(vec![1, 2, 3, 4, 5]);
    println!("Pipeline result: {:?}", transformed);

    println!("\n=== Mutex Basic ===");
    let count = mutex_basic();
    println!("Mutex count: {}", count);

    println!("\n=== Arc<Mutex<T>> Counter ===");
    let count = arc_mutex_counter(4, 250);
    println!("Arc<Mutex> counter: {}", count);

    println!("\n=== Arc<Mutex<T>> Shared Vec ===");
    let vec = arc_mutex_shared_vec(5);
    println!("Shared vec: {:?}", vec);

    println!("\n=== Parallel Word Count ===");
    let texts = vec![
        String::from("hello world"),
        String::from("rust is fast and safe"),
        String::from("concurrency without fear"),
    ];
    let total_words = parallel_word_count(texts);
    println!("Total words: {}", total_words);

    println!("\n=== Parallel Map ===");
    let numbers = vec![1, 2, 3, 4, 5];
    let squared = parallel_map(numbers, |x| x * x);
    println!("Squared: {:?}", squared);

    println!("\n=== Thread with Duration ===");
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });
    println!("Waiting for thread...");
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_basic_thread() {
        let msg = spawn_basic_thread();
        assert_eq!(msg, "Hello from thread!");
    }

    #[test]
    fn test_spawn_with_data() {
        let result = spawn_with_data(vec![1, 2, 3, 4, 5]);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_spawn_with_empty_data() {
        let result = spawn_with_data(vec![]);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_parallel_sum() {
        let numbers: Vec<i32> = (1..=100).collect();
        assert_eq!(parallel_sum(numbers, 25), 5050);
    }

    #[test]
    fn test_parallel_sum_small() {
        assert_eq!(parallel_sum(vec![1, 2, 3], 2), 6);
    }

    #[test]
    fn test_channel_basic() {
        let messages = channel_basic();
        assert_eq!(messages, vec!["hello", "from", "thread"]);
    }

    #[test]
    fn test_channel_multiple_producers() {
        let results = channel_multiple_producers();
        assert_eq!(results, vec![0, 10, 20]);
    }

    #[test]
    fn test_pipeline_transform() {
        let result = pipeline_transform(vec![1, 2, 3]);
        assert_eq!(result, vec![3, 5, 7]); // (x*2)+1
    }

    #[test]
    fn test_pipeline_empty() {
        let result = pipeline_transform(vec![]);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_mutex_basic() {
        assert_eq!(mutex_basic(), 2);
    }

    #[test]
    fn test_arc_mutex_counter() {
        let count = arc_mutex_counter(4, 100);
        assert_eq!(count, 400);
    }

    #[test]
    fn test_arc_mutex_counter_single_thread() {
        let count = arc_mutex_counter(1, 50);
        assert_eq!(count, 50);
    }

    #[test]
    fn test_arc_mutex_shared_vec() {
        let result = arc_mutex_shared_vec(3);
        assert_eq!(result, vec!["thread-0", "thread-1", "thread-2"]);
    }

    #[test]
    fn test_parallel_word_count() {
        let texts = vec![
            String::from("hello world"),        // 2
            String::from("one two three four"), // 4
        ];
        assert_eq!(parallel_word_count(texts), 6);
    }

    #[test]
    fn test_parallel_word_count_empty() {
        let texts = vec![String::from("")];
        assert_eq!(parallel_word_count(texts), 0);
    }

    #[test]
    fn test_parallel_map() {
        let result = parallel_map(vec![1, 2, 3], |x| x * x);
        assert_eq!(result, vec![1, 4, 9]);
    }

    #[test]
    fn test_parallel_map_strings() {
        let result = parallel_map(
            vec![String::from("hello"), String::from("world")],
            |s| s.len(),
        );
        assert_eq!(result, vec![5, 5]);
    }

    #[test]
    fn test_parallel_map_preserves_order() {
        let result = parallel_map(vec![5, 4, 3, 2, 1], |x| x * 10);
        assert_eq!(result, vec![50, 40, 30, 20, 10]);
    }
}
