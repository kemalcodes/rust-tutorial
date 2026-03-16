// Rust Tutorial #16: Channels and Message Passing
// Demonstrates tokio::sync::mpsc, oneshot, broadcast, and watch channels.
// Shows when to use each channel type and common async patterns.

use tokio::sync::{broadcast, mpsc, oneshot, watch};
use tokio::time::{sleep, Duration};

// ========================================================
// MPSC — Multiple Producer, Single Consumer
// ========================================================
// Many tasks can send messages, but only one task receives them.
// This is the most common channel type.

async fn mpsc_basic() -> Vec<String> {
    // Create a channel with buffer size 10
    let (tx, mut rx) = mpsc::channel::<String>(10);

    // Spawn a producer task
    tokio::spawn(async move {
        for i in 0..3 {
            tx.send(format!("message {}", i)).await.unwrap();
        }
        // tx is dropped here — the channel closes
    });

    // Collect all messages
    let mut messages = vec![];
    while let Some(msg) = rx.recv().await {
        messages.push(msg);
    }
    messages
}

// Multiple producers — clone the sender
async fn mpsc_multiple_producers() -> Vec<i32> {
    let (tx, mut rx) = mpsc::channel::<i32>(32);

    for i in 0..3 {
        let tx = tx.clone();
        tokio::spawn(async move {
            tx.send(i * 10).await.unwrap();
        });
    }

    // Drop the original sender so the channel closes
    // when all clones are dropped
    drop(tx);

    let mut results = vec![];
    while let Some(val) = rx.recv().await {
        results.push(val);
    }
    results.sort();
    results
}

// Bounded vs unbounded channels
async fn mpsc_bounded_backpressure() -> usize {
    // Small buffer — producer will wait when buffer is full
    let (tx, mut rx) = mpsc::channel::<i32>(2);
    let mut count = 0;

    let producer = tokio::spawn(async move {
        for i in 0..5 {
            // This will block when the buffer is full
            tx.send(i).await.unwrap();
        }
    });

    // Simulate slow consumer
    while let Some(_val) = rx.recv().await {
        count += 1;
        sleep(Duration::from_millis(10)).await;
    }

    producer.await.unwrap();
    count
}

// ========================================================
// ONESHOT — Single Value, One Producer, One Consumer
// ========================================================
// Send exactly one value. Good for request/response patterns.

async fn oneshot_basic() -> String {
    let (tx, rx) = oneshot::channel::<String>();

    tokio::spawn(async move {
        // Simulate some work
        sleep(Duration::from_millis(10)).await;
        tx.send("response".to_string()).unwrap();
    });

    // Wait for the single response
    rx.await.unwrap()
}

// Request/response pattern
struct Request {
    query: String,
    respond_to: oneshot::Sender<String>,
}

async fn request_response_pattern() -> String {
    let (tx, mut rx) = mpsc::channel::<Request>(10);

    // Spawn a "server" that handles requests
    tokio::spawn(async move {
        while let Some(req) = rx.recv().await {
            let response = format!("Result for: {}", req.query);
            let _ = req.respond_to.send(response);
        }
    });

    // Send a request and wait for response
    let (resp_tx, resp_rx) = oneshot::channel();
    tx.send(Request {
        query: "find users".to_string(),
        respond_to: resp_tx,
    })
    .await
    .unwrap();

    resp_rx.await.unwrap()
}

// ========================================================
// BROADCAST — Multiple Producers, Multiple Consumers
// ========================================================
// Every receiver gets a copy of every message.

async fn broadcast_basic() -> Vec<Vec<String>> {
    let (tx, _) = broadcast::channel::<String>(16);

    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    // Send messages
    tx.send("hello".to_string()).unwrap();
    tx.send("world".to_string()).unwrap();

    // Both receivers get both messages
    let mut results1 = vec![];
    let mut results2 = vec![];

    for _ in 0..2 {
        results1.push(rx1.recv().await.unwrap());
        results2.push(rx2.recv().await.unwrap());
    }

    vec![results1, results2]
}

// Event system using broadcast
async fn event_system() -> Vec<String> {
    let (tx, _) = broadcast::channel::<String>(16);

    let mut handles = vec![];

    // Spawn multiple listeners
    for id in 0..3 {
        let mut rx = tx.subscribe();
        let handle = tokio::spawn(async move {
            let msg = rx.recv().await.unwrap();
            format!("Listener {} got: {}", id, msg)
        });
        handles.push(handle);
    }

    // Small delay to ensure receivers are ready
    sleep(Duration::from_millis(10)).await;

    // Broadcast an event
    tx.send("user_logged_in".to_string()).unwrap();

    let mut results = vec![];
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results.sort();
    results
}

// ========================================================
// WATCH — Single Value, Latest State
// ========================================================
// Receivers always see the most recent value.
// Good for configuration or state that changes.

async fn watch_basic() -> Vec<String> {
    let (tx, mut rx) = watch::channel("initial".to_string());

    // The receiver always has the latest value
    let initial = rx.borrow().clone();

    tx.send("updated".to_string()).unwrap();
    // Wait for the change notification
    rx.changed().await.unwrap();
    let updated = rx.borrow().clone();

    tx.send("final".to_string()).unwrap();
    rx.changed().await.unwrap();
    let final_val = rx.borrow().clone();

    vec![initial, updated, final_val]
}

// Configuration watcher pattern
async fn config_watcher() -> String {
    let (tx, rx) = watch::channel("v1".to_string());

    // Spawn a task that reacts to config changes
    let mut task_rx = rx.clone();
    let handle = tokio::spawn(async move {
        task_rx.changed().await.unwrap();
        let new_config = task_rx.borrow().clone();
        format!("Config updated to: {}", new_config)
    });

    // Update the config
    sleep(Duration::from_millis(10)).await;
    tx.send("v2".to_string()).unwrap();

    handle.await.unwrap()
}

// ========================================================
// Channel Selection Patterns
// ========================================================

// Fan-in: multiple sources into one channel
async fn fan_in() -> Vec<String> {
    let (tx, mut rx) = mpsc::channel::<String>(32);

    // Source 1
    let tx1 = tx.clone();
    tokio::spawn(async move {
        tx1.send("from source 1".to_string()).await.unwrap();
    });

    // Source 2
    let tx2 = tx.clone();
    tokio::spawn(async move {
        tx2.send("from source 2".to_string()).await.unwrap();
    });

    drop(tx);

    let mut results = vec![];
    while let Some(msg) = rx.recv().await {
        results.push(msg);
    }
    results.sort();
    results
}

// Graceful shutdown with channels
async fn graceful_shutdown() -> Vec<String> {
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);
    let (result_tx, mut result_rx) = mpsc::channel::<String>(10);

    let result_tx_clone = result_tx.clone();
    let worker = tokio::spawn(async move {
        let mut count = 0;
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    result_tx_clone
                        .send(format!("Worker stopped after {} iterations", count))
                        .await
                        .unwrap();
                    break;
                }
                _ = sleep(Duration::from_millis(10)) => {
                    count += 1;
                    if count >= 3 {
                        // Signal that we've done enough work
                        result_tx_clone
                            .send(format!("iteration {}", count))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    });

    // Let the worker run for a bit
    sleep(Duration::from_millis(50)).await;

    // Send shutdown signal
    let _ = shutdown_tx.send(()).await;
    drop(shutdown_tx);
    drop(result_tx);

    worker.await.unwrap();

    let mut results = vec![];
    while let Some(msg) = result_rx.recv().await {
        results.push(msg);
    }
    results
}

#[tokio::main]
async fn main() {
    // MPSC
    let messages = mpsc_basic().await;
    println!("MPSC basic: {:?}", messages);

    let multi = mpsc_multiple_producers().await;
    println!("MPSC multiple producers: {:?}", multi);

    let count = mpsc_bounded_backpressure().await;
    println!("MPSC bounded: received {} messages", count);

    // Oneshot
    let response = oneshot_basic().await;
    println!("Oneshot: {}", response);

    let rr = request_response_pattern().await;
    println!("Request/Response: {}", rr);

    // Broadcast
    let results = broadcast_basic().await;
    println!("Broadcast: {:?}", results);

    let events = event_system().await;
    println!("Events: {:?}", events);

    // Watch
    let values = watch_basic().await;
    println!("Watch: {:?}", values);

    let config = config_watcher().await;
    println!("Config: {}", config);

    // Patterns
    let fanned = fan_in().await;
    println!("Fan-in: {:?}", fanned);

    let shutdown = graceful_shutdown().await;
    println!("Shutdown: {:?}", shutdown);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mpsc_basic() {
        let messages = mpsc_basic().await;
        assert_eq!(messages, vec!["message 0", "message 1", "message 2"]);
    }

    #[tokio::test]
    async fn test_mpsc_multiple_producers() {
        let results = mpsc_multiple_producers().await;
        assert_eq!(results, vec![0, 10, 20]);
    }

    #[tokio::test]
    async fn test_mpsc_bounded() {
        let count = mpsc_bounded_backpressure().await;
        assert_eq!(count, 5);
    }

    #[tokio::test]
    async fn test_oneshot_basic() {
        let result = oneshot_basic().await;
        assert_eq!(result, "response");
    }

    #[tokio::test]
    async fn test_request_response() {
        let result = request_response_pattern().await;
        assert_eq!(result, "Result for: find users");
    }

    #[tokio::test]
    async fn test_broadcast_basic() {
        let results = broadcast_basic().await;
        assert_eq!(results.len(), 2);
        assert_eq!(results[0], vec!["hello", "world"]);
        assert_eq!(results[1], vec!["hello", "world"]);
    }

    #[tokio::test]
    async fn test_event_system() {
        let results = event_system().await;
        assert_eq!(results.len(), 3);
        assert!(results[0].contains("got: user_logged_in"));
    }

    #[tokio::test]
    async fn test_watch_basic() {
        let values = watch_basic().await;
        assert_eq!(values, vec!["initial", "updated", "final"]);
    }

    #[tokio::test]
    async fn test_config_watcher() {
        let result = config_watcher().await;
        assert_eq!(result, "Config updated to: v2");
    }

    #[tokio::test]
    async fn test_fan_in() {
        let results = fan_in().await;
        assert_eq!(results, vec!["from source 1", "from source 2"]);
    }

    #[tokio::test]
    async fn test_graceful_shutdown() {
        let results = graceful_shutdown().await;
        assert!(!results.is_empty());
        assert!(results.last().unwrap().contains("Worker stopped"));
    }
}
