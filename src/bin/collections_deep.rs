// Rust Tutorial #26: Collections Deep Dive
// Demonstrates HashMap, BTreeMap, HashSet, BTreeSet, VecDeque, BinaryHeap,
// entry API, custom keys, and when to use which.

use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};

// ========================================================
// HashMap — Entry API
// ========================================================

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn group_by_length(words: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();
    for word in words {
        groups
            .entry(word.len())
            .or_insert_with(Vec::new)
            .push(word.to_string());
    }
    groups
}

fn merge_maps(a: &HashMap<String, i32>, b: &HashMap<String, i32>) -> HashMap<String, i32> {
    let mut result = a.clone();
    for (key, value) in b {
        *result.entry(key.clone()).or_insert(0) += value;
    }
    result
}

// ========================================================
// HashMap — Custom Keys
// ========================================================

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

fn grid_example() -> HashMap<Coordinate, String> {
    let mut grid = HashMap::new();
    grid.insert(Coordinate { x: 0, y: 0 }, "origin".to_string());
    grid.insert(Coordinate { x: 1, y: 0 }, "east".to_string());
    grid.insert(Coordinate { x: 0, y: 1 }, "north".to_string());
    grid
}

// ========================================================
// BTreeMap — Ordered Map
// ========================================================

fn sorted_word_count(text: &str) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for word in text.split_whitespace() {
        let word = word.to_lowercase();
        *counts.entry(word).or_insert(0) += 1;
    }
    counts
}

fn range_query(map: &BTreeMap<String, i32>, from: &str, to: &str) -> Vec<(String, i32)> {
    map.range(from.to_string()..=to.to_string())
        .map(|(k, v)| (k.clone(), *v))
        .collect()
}

fn top_scores(scores: &BTreeMap<String, u32>, n: usize) -> Vec<(String, u32)> {
    let mut sorted: Vec<_> = scores.iter().map(|(k, v)| (k.clone(), *v)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.into_iter().take(n).collect()
}

// ========================================================
// HashSet — Unique Values
// ========================================================

fn unique_words(text: &str) -> HashSet<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .collect()
}

fn set_operations(a: &HashSet<String>, b: &HashSet<String>) -> SetResult {
    SetResult {
        union: a.union(b).cloned().collect(),
        intersection: a.intersection(b).cloned().collect(),
        difference: a.difference(b).cloned().collect(),
        symmetric_difference: a.symmetric_difference(b).cloned().collect(),
    }
}

#[derive(Debug)]
struct SetResult {
    union: HashSet<String>,
    intersection: HashSet<String>,
    difference: HashSet<String>,
    symmetric_difference: HashSet<String>,
}

fn has_duplicates<T: std::hash::Hash + Eq>(items: &[T]) -> bool {
    let mut seen = HashSet::new();
    for item in items {
        if !seen.insert(item) {
            return true;
        }
    }
    false
}

// ========================================================
// BTreeSet — Ordered Set
// ========================================================

fn sorted_unique_chars(text: &str) -> BTreeSet<char> {
    text.chars().filter(|c| c.is_alphabetic()).collect()
}

fn range_set(set: &BTreeSet<i32>, from: i32, to: i32) -> Vec<i32> {
    set.range(from..=to).copied().collect()
}

// ========================================================
// VecDeque — Double-ended Queue
// ========================================================

fn sliding_window_average(numbers: &[f64], window_size: usize) -> Vec<f64> {
    let mut window: VecDeque<f64> = VecDeque::new();
    let mut averages = Vec::new();

    for &num in numbers {
        window.push_back(num);
        if window.len() > window_size {
            window.pop_front();
        }
        if window.len() == window_size {
            let avg = window.iter().sum::<f64>() / window_size as f64;
            averages.push(avg);
        }
    }

    averages
}

fn recent_history<T: Clone>(capacity: usize) -> RecentHistory<T> {
    RecentHistory {
        items: VecDeque::new(),
        capacity,
    }
}

struct RecentHistory<T> {
    items: VecDeque<T>,
    capacity: usize,
}

impl<T: Clone> RecentHistory<T> {
    fn add(&mut self, item: T) {
        if self.items.len() >= self.capacity {
            self.items.pop_front();
        }
        self.items.push_back(item);
    }

    fn get_all(&self) -> Vec<T> {
        self.items.iter().cloned().collect()
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

// ========================================================
// BinaryHeap — Priority Queue
// ========================================================

fn top_n_largest(numbers: &[i32], n: usize) -> Vec<i32> {
    let mut heap = BinaryHeap::from(numbers.to_vec());
    let mut result = Vec::new();
    for _ in 0..n.min(numbers.len()) {
        if let Some(val) = heap.pop() {
            result.push(val);
        }
    }
    result
}

fn sort_with_heap(numbers: &[i32]) -> Vec<i32> {
    let heap = BinaryHeap::from(numbers.to_vec());
    heap.into_sorted_vec()
}

#[derive(Debug, Eq, PartialEq)]
struct Task {
    priority: u32,
    name: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn process_tasks(tasks: Vec<Task>) -> Vec<String> {
    let mut heap = BinaryHeap::from(tasks);
    let mut order = Vec::new();
    while let Some(task) = heap.pop() {
        order.push(format!("[{}] {}", task.priority, task.name));
    }
    order
}

// ========================================================
// Practical Example: Frequency Analysis
// ========================================================

fn most_frequent<T: std::hash::Hash + Eq + Clone>(items: &[T], n: usize) -> Vec<(T, usize)> {
    let mut counts: HashMap<&T, usize> = HashMap::new();
    for item in items {
        *counts.entry(item).or_insert(0) += 1;
    }

    let mut sorted: Vec<_> = counts.into_iter().map(|(k, v)| (k.clone(), v)).collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    sorted.into_iter().take(n).collect()
}

// ========================================================
// Practical Example: Graph Adjacency List
// ========================================================

fn build_graph(edges: &[(String, String)]) -> HashMap<String, Vec<String>> {
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for (from, to) in edges {
        graph.entry(from.clone()).or_default().push(to.clone());
        graph.entry(to.clone()).or_default().push(from.clone());
    }
    graph
}

fn neighbors(graph: &HashMap<String, Vec<String>>, node: &str) -> Vec<String> {
    graph.get(node).cloned().unwrap_or_default()
}

fn main() {
    println!("=== Collections Deep Dive ===\n");

    // Demo 1: Word count with HashMap
    println!("--- HashMap: Word Count ---");
    let text = "the cat sat on the mat the cat";
    let counts = word_count(text);
    println!("Text: {:?}", text);
    for (word, count) in &counts {
        println!("  {} -> {}", word, count);
    }

    println!();

    // Demo 2: Group by length
    println!("--- HashMap: Group by Length ---");
    let words = vec!["cat", "dog", "fish", "ant", "bear", "ox"];
    let groups = group_by_length(&words);
    for (len, words) in &groups {
        println!("  Length {}: {:?}", len, words);
    }

    println!();

    // Demo 3: Merge maps
    println!("--- HashMap: Merge ---");
    let mut a = HashMap::new();
    a.insert("x".to_string(), 10);
    a.insert("y".to_string(), 20);
    let mut b = HashMap::new();
    b.insert("y".to_string(), 5);
    b.insert("z".to_string(), 15);
    let merged = merge_maps(&a, &b);
    println!("Merged: {:?}", merged);

    println!();

    // Demo 4: Custom keys
    println!("--- HashMap: Custom Keys ---");
    let grid = grid_example();
    let origin = grid.get(&Coordinate { x: 0, y: 0 });
    println!("Origin: {:?}", origin);

    println!();

    // Demo 5: BTreeMap
    println!("--- BTreeMap: Sorted ---");
    let sorted = sorted_word_count(text);
    for (word, count) in &sorted {
        println!("  {} -> {}", word, count);
    }

    println!();

    // Demo 6: BTreeMap range query
    println!("--- BTreeMap: Range Query ---");
    let mut map = BTreeMap::new();
    map.insert("apple".to_string(), 1);
    map.insert("banana".to_string(), 2);
    map.insert("cherry".to_string(), 3);
    map.insert("date".to_string(), 4);
    map.insert("elderberry".to_string(), 5);
    let range = range_query(&map, "banana", "date");
    println!("Range [banana, date]: {:?}", range);

    println!();

    // Demo 7: HashSet
    println!("--- HashSet ---");
    let set_a: HashSet<String> = ["rust", "go", "python"].iter().map(|s| s.to_string()).collect();
    let set_b: HashSet<String> = ["python", "java", "rust"].iter().map(|s| s.to_string()).collect();
    let ops = set_operations(&set_a, &set_b);
    println!("A: {:?}", set_a);
    println!("B: {:?}", set_b);
    println!("Intersection: {:?}", ops.intersection);
    println!("Difference (A-B): {:?}", ops.difference);

    println!();

    // Demo 8: Duplicates
    println!("--- Has Duplicates ---");
    println!("[1,2,3]: {}", has_duplicates(&[1, 2, 3]));
    println!("[1,2,1]: {}", has_duplicates(&[1, 2, 1]));

    println!();

    // Demo 9: BTreeSet
    println!("--- BTreeSet ---");
    let chars = sorted_unique_chars("hello world");
    println!("Sorted unique chars: {:?}", chars);

    let num_set: BTreeSet<i32> = [5, 2, 8, 1, 9, 3, 7].into_iter().collect();
    let range = range_set(&num_set, 3, 7);
    println!("Range [3,7]: {:?}", range);

    println!();

    // Demo 10: VecDeque
    println!("--- VecDeque: Sliding Window ---");
    let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
    let averages = sliding_window_average(&data, 3);
    println!("Data: {:?}", data);
    println!("Window(3) averages: {:?}", averages);

    println!();

    // Demo 11: Recent history
    println!("--- VecDeque: Recent History ---");
    let mut history: RecentHistory<String> = recent_history(3);
    history.add("page1".to_string());
    history.add("page2".to_string());
    history.add("page3".to_string());
    history.add("page4".to_string());
    println!("Recent (cap 3): {:?}", history.get_all());

    println!();

    // Demo 12: BinaryHeap
    println!("--- BinaryHeap: Top N ---");
    let nums = vec![42, 17, 95, 3, 88, 61, 25];
    let top3 = top_n_largest(&nums, 3);
    println!("Numbers: {:?}", nums);
    println!("Top 3: {:?}", top3);

    println!();

    // Demo 13: Heap sort
    println!("--- BinaryHeap: Sort ---");
    let sorted = sort_with_heap(&nums);
    println!("Sorted: {:?}", sorted);

    println!();

    // Demo 14: Task priority queue
    println!("--- BinaryHeap: Task Queue ---");
    let tasks = vec![
        Task { priority: 3, name: "Send email".to_string() },
        Task { priority: 1, name: "Check logs".to_string() },
        Task { priority: 5, name: "Fix bug".to_string() },
        Task { priority: 2, name: "Write docs".to_string() },
    ];
    let order = process_tasks(tasks);
    for task in &order {
        println!("  {}", task);
    }

    println!();

    // Demo 15: Frequency analysis
    println!("--- Most Frequent ---");
    let letters: Vec<char> = "abracadabra".chars().collect();
    let freq = most_frequent(&letters, 3);
    println!("Most frequent in 'abracadabra': {:?}", freq);

    println!();

    // Demo 16: Graph
    println!("--- Graph ---");
    let edges = vec![
        ("A".to_string(), "B".to_string()),
        ("A".to_string(), "C".to_string()),
        ("B".to_string(), "C".to_string()),
        ("C".to_string(), "D".to_string()),
    ];
    let graph = build_graph(&edges);
    println!("Neighbors of A: {:?}", neighbors(&graph, "A"));
    println!("Neighbors of C: {:?}", neighbors(&graph, "C"));

    println!("\n=== All demos completed! ===");
}

// ========================================================
// Tests
// ========================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- HashMap tests ---

    #[test]
    fn test_word_count() {
        let counts = word_count("the cat sat on the mat");
        assert_eq!(counts["the"], 2);
        assert_eq!(counts["cat"], 1);
        assert_eq!(counts["mat"], 1);
    }

    #[test]
    fn test_word_count_case_insensitive() {
        let counts = word_count("Hello hello HELLO");
        assert_eq!(counts["hello"], 3);
    }

    #[test]
    fn test_group_by_length() {
        let words = vec!["cat", "dog", "fish", "ant"];
        let groups = group_by_length(&words);
        assert_eq!(groups[&3].len(), 3); // cat, dog, ant
        assert_eq!(groups[&4].len(), 1); // fish
    }

    #[test]
    fn test_merge_maps() {
        let mut a = HashMap::new();
        a.insert("x".to_string(), 10);
        a.insert("y".to_string(), 20);
        let mut b = HashMap::new();
        b.insert("y".to_string(), 5);
        b.insert("z".to_string(), 15);
        let merged = merge_maps(&a, &b);
        assert_eq!(merged["x"], 10);
        assert_eq!(merged["y"], 25);
        assert_eq!(merged["z"], 15);
    }

    #[test]
    fn test_grid_example() {
        let grid = grid_example();
        assert_eq!(grid[&Coordinate { x: 0, y: 0 }], "origin");
        assert_eq!(grid[&Coordinate { x: 1, y: 0 }], "east");
    }

    // --- BTreeMap tests ---

    #[test]
    fn test_sorted_word_count() {
        let counts = sorted_word_count("b a c a b a");
        let keys: Vec<_> = counts.keys().collect();
        assert_eq!(keys, vec!["a", "b", "c"]);
        assert_eq!(counts["a"], 3);
    }

    #[test]
    fn test_range_query() {
        let mut map = BTreeMap::new();
        map.insert("a".to_string(), 1);
        map.insert("b".to_string(), 2);
        map.insert("c".to_string(), 3);
        map.insert("d".to_string(), 4);
        let range = range_query(&map, "b", "c");
        assert_eq!(range, vec![("b".to_string(), 2), ("c".to_string(), 3)]);
    }

    #[test]
    fn test_top_scores() {
        let mut scores = BTreeMap::new();
        scores.insert("Alex".to_string(), 95);
        scores.insert("Sam".to_string(), 87);
        scores.insert("Jordan".to_string(), 92);
        let top = top_scores(&scores, 2);
        assert_eq!(top[0].0, "Alex");
        assert_eq!(top[1].0, "Jordan");
    }

    // --- HashSet tests ---

    #[test]
    fn test_unique_words() {
        let unique = unique_words("hello world hello");
        assert_eq!(unique.len(), 2);
        assert!(unique.contains("hello"));
        assert!(unique.contains("world"));
    }

    #[test]
    fn test_set_operations() {
        let a: HashSet<String> = ["x", "y", "z"].iter().map(|s| s.to_string()).collect();
        let b: HashSet<String> = ["y", "z", "w"].iter().map(|s| s.to_string()).collect();
        let ops = set_operations(&a, &b);
        assert_eq!(ops.union.len(), 4);
        assert_eq!(ops.intersection.len(), 2);
        assert!(ops.intersection.contains("y"));
        assert!(ops.intersection.contains("z"));
        assert_eq!(ops.difference.len(), 1);
        assert!(ops.difference.contains("x"));
        assert_eq!(ops.symmetric_difference.len(), 2);
    }

    #[test]
    fn test_has_duplicates_true() {
        assert!(has_duplicates(&[1, 2, 3, 2]));
    }

    #[test]
    fn test_has_duplicates_false() {
        assert!(!has_duplicates(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_has_duplicates_empty() {
        let empty: Vec<i32> = vec![];
        assert!(!has_duplicates(&empty));
    }

    // --- BTreeSet tests ---

    #[test]
    fn test_sorted_unique_chars() {
        let chars = sorted_unique_chars("hello");
        let expected: BTreeSet<char> = ['e', 'h', 'l', 'o'].into_iter().collect();
        assert_eq!(chars, expected);
    }

    #[test]
    fn test_range_set() {
        let set: BTreeSet<i32> = [1, 3, 5, 7, 9].into_iter().collect();
        let range = range_set(&set, 3, 7);
        assert_eq!(range, vec![3, 5, 7]);
    }

    // --- VecDeque tests ---

    #[test]
    fn test_sliding_window_average() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let avgs = sliding_window_average(&data, 3);
        assert_eq!(avgs.len(), 3);
        assert!((avgs[0] - 2.0).abs() < 0.001);
        assert!((avgs[1] - 3.0).abs() < 0.001);
        assert!((avgs[2] - 4.0).abs() < 0.001);
    }

    #[test]
    fn test_recent_history() {
        let mut h: RecentHistory<i32> = recent_history(3);
        h.add(1);
        h.add(2);
        h.add(3);
        assert_eq!(h.get_all(), vec![1, 2, 3]);
        h.add(4);
        assert_eq!(h.get_all(), vec![2, 3, 4]);
        assert_eq!(h.len(), 3);
    }

    #[test]
    fn test_recent_history_under_capacity() {
        let mut h: RecentHistory<i32> = recent_history(5);
        h.add(1);
        h.add(2);
        assert_eq!(h.len(), 2);
        assert_eq!(h.get_all(), vec![1, 2]);
    }

    // --- BinaryHeap tests ---

    #[test]
    fn test_top_n_largest() {
        let nums = vec![42, 17, 95, 3, 88];
        let top2 = top_n_largest(&nums, 2);
        assert_eq!(top2, vec![95, 88]);
    }

    #[test]
    fn test_top_n_larger_than_list() {
        let nums = vec![5, 3, 1];
        let top5 = top_n_largest(&nums, 5);
        assert_eq!(top5, vec![5, 3, 1]);
    }

    #[test]
    fn test_sort_with_heap() {
        let nums = vec![5, 3, 1, 4, 2];
        let sorted = sort_with_heap(&nums);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_process_tasks() {
        let tasks = vec![
            Task { priority: 1, name: "Low".to_string() },
            Task { priority: 3, name: "High".to_string() },
            Task { priority: 2, name: "Medium".to_string() },
        ];
        let order = process_tasks(tasks);
        assert_eq!(order[0], "[3] High");
        assert_eq!(order[1], "[2] Medium");
        assert_eq!(order[2], "[1] Low");
    }

    // --- Frequency analysis tests ---

    #[test]
    fn test_most_frequent() {
        let items = vec!['a', 'b', 'a', 'c', 'a', 'b'];
        let freq = most_frequent(&items, 2);
        assert_eq!(freq[0], ('a', 3));
        assert_eq!(freq[1], ('b', 2));
    }

    #[test]
    fn test_most_frequent_single() {
        let items = vec![1, 1, 1];
        let freq = most_frequent(&items, 5);
        assert_eq!(freq.len(), 1);
        assert_eq!(freq[0], (1, 3));
    }

    // --- Graph tests ---

    #[test]
    fn test_build_graph() {
        let edges = vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
        ];
        let graph = build_graph(&edges);
        assert_eq!(graph["A"].len(), 2);
        assert_eq!(graph["B"].len(), 1);
        assert_eq!(graph["C"].len(), 1);
    }

    #[test]
    fn test_neighbors() {
        let edges = vec![
            ("A".to_string(), "B".to_string()),
            ("A".to_string(), "C".to_string()),
        ];
        let graph = build_graph(&edges);
        let n = neighbors(&graph, "A");
        assert_eq!(n.len(), 2);
        assert!(n.contains(&"B".to_string()));
        assert!(n.contains(&"C".to_string()));
    }

    #[test]
    fn test_neighbors_unknown() {
        let graph: HashMap<String, Vec<String>> = HashMap::new();
        let n = neighbors(&graph, "X");
        assert!(n.is_empty());
    }
}
