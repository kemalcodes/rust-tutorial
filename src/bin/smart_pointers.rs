// Rust Tutorial #13: Smart Pointers — Box, Rc, Arc
// Demonstrates Box<T>, Deref trait, Drop trait, Rc<T>, Arc<T>,
// RefCell<T>, interior mutability, and when to use which.

use std::cell::RefCell;
use std::fmt;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;

// --- Box<T>: heap allocation ---

// Recursive type — needs Box because size is unknown at compile time
#[derive(Debug, PartialEq)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::Nil
    }

    fn push(self, value: T) -> List<T> {
        List::Cons(value, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Nil => 0,
            List::Cons(_, rest) => 1 + rest.len(),
        }
    }

    fn to_vec(&self) -> Vec<&T> {
        let mut result = Vec::new();
        let mut current = self;
        while let List::Cons(value, next) = current {
            result.push(value);
            current = next;
        }
        result
    }
}

// --- Deref trait: custom smart pointer ---

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> MyBox<T> {
        MyBox(value)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

// --- Drop trait: cleanup on drop ---

struct Resource {
    name: String,
    log: Rc<RefCell<Vec<String>>>,
}

impl Resource {
    fn new(name: &str, log: Rc<RefCell<Vec<String>>>) -> Resource {
        log.borrow_mut().push(format!("Created: {}", name));
        Resource {
            name: name.to_string(),
            log,
        }
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        self.log
            .borrow_mut()
            .push(format!("Dropped: {}", self.name));
    }
}

// --- Rc<T>: reference counting ---

#[derive(Debug)]
struct SharedData {
    value: String,
}

impl SharedData {
    fn new(value: &str) -> SharedData {
        SharedData {
            value: value.to_string(),
        }
    }
}

// Tree node using Rc for shared children
#[derive(Debug)]
struct TreeNode {
    value: i32,
    children: Vec<Rc<TreeNode>>,
}

impl TreeNode {
    fn new(value: i32) -> TreeNode {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    fn with_children(value: i32, children: Vec<Rc<TreeNode>>) -> TreeNode {
        TreeNode { value, children }
    }

    fn sum(&self) -> i32 {
        let children_sum: i32 = self.children.iter().map(|c| c.sum()).sum();
        self.value + children_sum
    }
}

// --- RefCell<T>: interior mutability ---

#[derive(Debug)]
struct Counter {
    count: RefCell<u32>,
    name: String,
}

impl Counter {
    fn new(name: &str) -> Counter {
        Counter {
            count: RefCell::new(0),
            name: name.to_string(),
        }
    }

    fn increment(&self) {
        *self.count.borrow_mut() += 1;
    }

    fn get(&self) -> u32 {
        *self.count.borrow()
    }
}

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.count.borrow())
    }
}

// Rc<RefCell<T>> pattern — shared mutable data
fn create_shared_list() -> (Rc<RefCell<Vec<String>>>, Rc<RefCell<Vec<String>>>) {
    let data = Rc::new(RefCell::new(vec![String::from("initial")]));
    let clone1 = Rc::clone(&data);
    let clone2 = Rc::clone(&data);
    (clone1, clone2)
}

fn main() {
    println!("=== Box<T>: Heap Allocation ===");
    // Simple heap allocation
    let boxed = Box::new(42);
    println!("Boxed value: {}", boxed);
    println!("Boxed + 1: {}", *boxed + 1);

    // Large data on the heap
    let big_array = Box::new([0u8; 1000]);
    println!("Array length: {}", big_array.len());

    // Recursive type with Box
    let list = List::new().push(3).push(2).push(1);
    println!("List length: {}", list.len());
    println!("List values: {:?}", list.to_vec());

    println!("\n=== Deref Trait ===");
    let my_box = MyBox::new(String::from("hello"));
    // Deref coercion: &MyBox<String> -> &String -> &str
    println!("MyBox value: {}", *my_box);
    // This works because of deref coercion:
    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    greet(&my_box);

    println!("\n=== Drop Trait ===");
    let log = Rc::new(RefCell::new(Vec::new()));
    {
        let _r1 = Resource::new("file.txt", Rc::clone(&log));
        let _r2 = Resource::new("db_conn", Rc::clone(&log));
        // r2 is dropped first (reverse order), then r1
    }
    println!("Log: {:?}", log.borrow());

    // Explicit early drop
    let log2 = Rc::new(RefCell::new(Vec::new()));
    let r3 = Resource::new("temp", Rc::clone(&log2));
    drop(r3); // Drop early
    println!("After explicit drop: {:?}", log2.borrow());

    println!("\n=== Rc<T>: Reference Counting ===");
    let shared = Rc::new(SharedData::new("important"));
    let clone1 = Rc::clone(&shared);
    let clone2 = Rc::clone(&shared);
    println!("Reference count: {}", Rc::strong_count(&shared));
    println!("Value: {}", shared.value);
    println!("Clone1: {}", clone1.value);
    println!("Clone2: {}", clone2.value);
    drop(clone1);
    println!("After drop: count = {}", Rc::strong_count(&shared));

    // Tree with shared nodes
    let leaf = Rc::new(TreeNode::new(10));
    let branch1 = Rc::new(TreeNode::with_children(5, vec![Rc::clone(&leaf)]));
    let branch2 = Rc::new(TreeNode::with_children(3, vec![Rc::clone(&leaf)]));
    let root = TreeNode::with_children(1, vec![branch1, branch2]);
    println!("Tree sum: {}", root.sum()); // 1 + 5 + 10 + 3 + 10 = 29

    println!("\n=== RefCell<T>: Interior Mutability ===");
    let counter = Counter::new("clicks");
    counter.increment(); // No &mut needed!
    counter.increment();
    counter.increment();
    println!("{}", counter);
    println!("Count value: {}", counter.get());

    // Rc<RefCell<T>> pattern
    let (handle1, handle2) = create_shared_list();
    handle1.borrow_mut().push(String::from("from handle1"));
    handle2.borrow_mut().push(String::from("from handle2"));
    println!("Shared list: {:?}", handle1.borrow());

    println!("\n=== Arc<T>: Thread-Safe Reference Counting ===");
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let data_clone = Arc::clone(&data);

    let handle = std::thread::spawn(move || {
        let sum: i32 = data_clone.iter().sum();
        println!("Sum from thread: {}", sum);
        sum
    });

    println!("Original data: {:?}", data);
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box_basic() {
        let boxed = Box::new(42);
        assert_eq!(*boxed, 42);
    }

    #[test]
    fn test_list_empty() {
        let list: List<i32> = List::new();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_push() {
        let list = List::new().push(3).push(2).push(1);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_list_to_vec() {
        let list = List::new().push(3).push(2).push(1);
        let values = list.to_vec();
        assert_eq!(values, vec![&1, &2, &3]);
    }

    #[test]
    fn test_my_box_deref() {
        let my_box = MyBox::new(42);
        assert_eq!(*my_box, 42);
    }

    #[test]
    fn test_drop_order() {
        let log = Rc::new(RefCell::new(Vec::new()));
        {
            let _r1 = Resource::new("first", Rc::clone(&log));
            let _r2 = Resource::new("second", Rc::clone(&log));
        }
        let entries = log.borrow();
        assert_eq!(entries.len(), 4);
        assert_eq!(entries[0], "Created: first");
        assert_eq!(entries[1], "Created: second");
        assert_eq!(entries[2], "Dropped: second"); // Reverse order
        assert_eq!(entries[3], "Dropped: first");
    }

    #[test]
    fn test_explicit_drop() {
        let log = Rc::new(RefCell::new(Vec::new()));
        let r = Resource::new("temp", Rc::clone(&log));
        assert_eq!(log.borrow().len(), 1);
        drop(r);
        assert_eq!(log.borrow().len(), 2);
        assert_eq!(log.borrow()[1], "Dropped: temp");
    }

    #[test]
    fn test_rc_reference_count() {
        let data = Rc::new(42);
        assert_eq!(Rc::strong_count(&data), 1);
        let clone1 = Rc::clone(&data);
        assert_eq!(Rc::strong_count(&data), 2);
        drop(clone1);
        assert_eq!(Rc::strong_count(&data), 1);
    }

    #[test]
    fn test_rc_shared_value() {
        let data = Rc::new(String::from("shared"));
        let clone = Rc::clone(&data);
        assert_eq!(*data, "shared");
        assert_eq!(*clone, "shared");
    }

    #[test]
    fn test_tree_sum() {
        let leaf = Rc::new(TreeNode::new(10));
        let branch = Rc::new(TreeNode::with_children(5, vec![Rc::clone(&leaf)]));
        let root = TreeNode::with_children(1, vec![branch]);
        assert_eq!(root.sum(), 16);
    }

    #[test]
    fn test_tree_shared_leaf() {
        let leaf = Rc::new(TreeNode::new(10));
        let root = TreeNode::with_children(
            1,
            vec![Rc::clone(&leaf), Rc::clone(&leaf)],
        );
        assert_eq!(root.sum(), 21); // 1 + 10 + 10
    }

    #[test]
    fn test_counter_increment() {
        let counter = Counter::new("test");
        assert_eq!(counter.get(), 0);
        counter.increment();
        counter.increment();
        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_counter_display() {
        let counter = Counter::new("clicks");
        counter.increment();
        assert_eq!(format!("{}", counter), "clicks: 1");
    }

    #[test]
    fn test_refcell_shared_mutation() {
        let (h1, h2) = create_shared_list();
        h1.borrow_mut().push(String::from("a"));
        h2.borrow_mut().push(String::from("b"));
        let data = h1.borrow();
        assert_eq!(data.len(), 3);
        assert_eq!(data[0], "initial");
        assert_eq!(data[1], "a");
        assert_eq!(data[2], "b");
    }

    #[test]
    fn test_arc_thread_safety() {
        let data = Arc::new(vec![1, 2, 3, 4, 5]);
        let data_clone = Arc::clone(&data);

        let handle = std::thread::spawn(move || {
            data_clone.iter().sum::<i32>()
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 15);
        assert_eq!(*data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_arc_reference_count() {
        let data = Arc::new(42);
        assert_eq!(Arc::strong_count(&data), 1);
        let clone = Arc::clone(&data);
        assert_eq!(Arc::strong_count(&data), 2);
        drop(clone);
        assert_eq!(Arc::strong_count(&data), 1);
    }
}
