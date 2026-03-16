// Rust Tutorial #15: Async/Await and Tokio
// Demonstrates async fn, .await, #[tokio::main], tokio::spawn,
// tokio::join!, tokio::select!, tokio::time::sleep, and async vs threads.

use std::time::Duration;
use tokio::time::sleep;

// --- Basic async function ---
// An async fn returns a Future. The function body does not run
// until you .await the future.

async fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// --- Simulating async work with sleep ---

async fn fetch_user(id: u32) -> String {
    // Simulate a network request
    sleep(Duration::from_millis(100)).await;
    format!("User(id={})", id)
}

async fn fetch_order(id: u32) -> String {
    sleep(Duration::from_millis(80)).await;
    format!("Order(id={})", id)
}

// --- Running tasks sequentially ---

async fn sequential_requests() -> (String, String) {
    let user = fetch_user(1).await;
    let order = fetch_order(42).await;
    (user, order)
}

// --- Running tasks concurrently with tokio::join! ---
// tokio::join! runs multiple futures at the same time on the same task.
// It waits for ALL of them to finish.

async fn concurrent_requests() -> (String, String) {
    let (user, order) = tokio::join!(
        fetch_user(1),
        fetch_order(42)
    );
    (user, order)
}

// --- Spawning independent tasks with tokio::spawn ---
// tokio::spawn creates a new task that runs on the Tokio runtime.
// Unlike tokio::join!, each spawned task runs independently.

async fn spawn_tasks() -> Vec<String> {
    let mut handles = vec![];

    for i in 0..3 {
        let handle = tokio::spawn(async move {
            sleep(Duration::from_millis(50)).await;
            format!("Task {} done", i)
        });
        handles.push(handle);
    }

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results
}

// --- tokio::select! — race multiple futures ---
// select! waits for the FIRST future to complete and cancels the rest.

async fn timeout_example() -> &'static str {
    tokio::select! {
        _ = sleep(Duration::from_secs(10)) => {
            "slow task finished"
        }
        _ = sleep(Duration::from_millis(50)) => {
            "timeout reached"
        }
    }
}

// --- Async channels (covered more in RS-16) ---

async fn simple_channel() -> String {
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(10);

    tokio::spawn(async move {
        tx.send("hello from task".to_string()).await.unwrap();
    });

    rx.recv().await.unwrap()
}

// --- Returning errors from async functions ---

async fn fetch_data(url: &str) -> Result<String, String> {
    if url.is_empty() {
        return Err("URL cannot be empty".to_string());
    }
    sleep(Duration::from_millis(10)).await;
    Ok(format!("Data from {}", url))
}

// --- Async closures and blocks ---

async fn async_block_example() -> i32 {
    let future = async {
        let a = 10;
        let b = 20;
        a + b
    };
    future.await
}

// --- Shared state across tasks ---

async fn shared_counter() -> u32 {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    let counter = Arc::new(Mutex::new(0u32));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            let mut lock = counter.lock().await;
            *lock += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await.unwrap();
    }

    let result = *counter.lock().await;
    result
}

// --- Main function using #[tokio::main] ---
// This macro sets up the Tokio runtime and runs your async main function.

#[tokio::main]
async fn main() {
    // Basic async function
    let greeting = greet("Alex").await;
    println!("{}", greeting);

    // Sequential requests
    let (user, order) = sequential_requests().await;
    println!("Sequential: {} and {}", user, order);

    // Concurrent requests
    let (user, order) = concurrent_requests().await;
    println!("Concurrent: {} and {}", user, order);

    // Spawned tasks
    let results = spawn_tasks().await;
    println!("Spawned: {:?}", results);

    // Select (timeout)
    let winner = timeout_example().await;
    println!("Select winner: {}", winner);

    // Simple channel
    let msg = simple_channel().await;
    println!("Channel: {}", msg);

    // Error handling
    match fetch_data("https://example.com").await {
        Ok(data) => println!("Got: {}", data),
        Err(e) => println!("Error: {}", e),
    }

    // Async block
    let sum = async_block_example().await;
    println!("Async block: {}", sum);

    // Shared counter
    let count = shared_counter().await;
    println!("Counter: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_greet() {
        let result = greet("Sam").await;
        assert_eq!(result, "Hello, Sam!");
    }

    #[tokio::test]
    async fn test_fetch_user() {
        let result = fetch_user(5).await;
        assert_eq!(result, "User(id=5)");
    }

    #[tokio::test]
    async fn test_sequential_requests() {
        let (user, order) = sequential_requests().await;
        assert_eq!(user, "User(id=1)");
        assert_eq!(order, "Order(id=42)");
    }

    #[tokio::test]
    async fn test_concurrent_requests() {
        let (user, order) = concurrent_requests().await;
        assert_eq!(user, "User(id=1)");
        assert_eq!(order, "Order(id=42)");
    }

    #[tokio::test]
    async fn test_spawn_tasks() {
        let results = spawn_tasks().await;
        assert_eq!(results.len(), 3);
        // Tasks may finish in any order, so check contents
        assert!(results.contains(&"Task 0 done".to_string()));
        assert!(results.contains(&"Task 1 done".to_string()));
        assert!(results.contains(&"Task 2 done".to_string()));
    }

    #[tokio::test]
    async fn test_timeout_example() {
        let result = timeout_example().await;
        assert_eq!(result, "timeout reached");
    }

    #[tokio::test]
    async fn test_simple_channel() {
        let result = simple_channel().await;
        assert_eq!(result, "hello from task");
    }

    #[tokio::test]
    async fn test_fetch_data_ok() {
        let result = fetch_data("https://example.com").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Data from https://example.com");
    }

    #[tokio::test]
    async fn test_fetch_data_error() {
        let result = fetch_data("").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_async_block() {
        let result = async_block_example().await;
        assert_eq!(result, 30);
    }

    #[tokio::test]
    async fn test_shared_counter() {
        let result = shared_counter().await;
        assert_eq!(result, 5);
    }
}
