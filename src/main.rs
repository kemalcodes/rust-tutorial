// Rust Tutorial #7: Enums and Pattern Matching
// Demonstrates enums, enums with data, match expressions, if let, while let,
// Option<T>, and combining enums with structs.

use std::f64::consts::PI;

// --- Simple Enum ---
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::East | Direction::West)
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

// --- Enum with Data ---
#[derive(Debug, PartialEq)]
enum Message {
    Quit,
    Echo(String),
    Move { x: i32, y: i32 },
    Color(u8, u8, u8),
}

impl Message {
    fn describe(&self) -> String {
        match self {
            Message::Quit => String::from("quit"),
            Message::Echo(text) => format!("echo: {}", text),
            Message::Move { x, y } => format!("move to ({}, {})", x, y),
            Message::Color(r, g, b) => format!("color: rgb({}, {}, {})", r, g, b),
        }
    }

    fn is_quit(&self) -> bool {
        matches!(self, Message::Quit)
    }
}

// --- Enum with Methods (Shape) ---
#[derive(Debug, PartialEq)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(a, b, c) => {
                let s = (a + b + c) / 2.0;
                (s * (s - a) * (s - b) * (s - c)).sqrt()
            }
        }
    }

    fn name(&self) -> &str {
        match self {
            Shape::Circle(_) => "circle",
            Shape::Rectangle(_, _) => "rectangle",
            Shape::Triangle(_, _, _) => "triangle",
        }
    }
}

// --- Enum + Struct Combination ---
#[derive(Debug, PartialEq)]
enum OrderStatus {
    Pending,
    Shipped(String),
    Delivered,
    Cancelled(String),
}

#[derive(Debug)]
struct Order {
    id: u32,
    item: String,
    status: OrderStatus,
}

impl Order {
    fn new(id: u32, item: String) -> Order {
        Order {
            id,
            item,
            status: OrderStatus::Pending,
        }
    }

    fn ship(&mut self, tracking: String) {
        self.status = OrderStatus::Shipped(tracking);
    }

    fn deliver(&mut self) {
        self.status = OrderStatus::Delivered;
    }

    fn cancel(&mut self, reason: String) {
        self.status = OrderStatus::Cancelled(reason);
    }

    fn status_text(&self) -> String {
        match &self.status {
            OrderStatus::Pending => String::from("pending"),
            OrderStatus::Shipped(tracking) => format!("shipped ({})", tracking),
            OrderStatus::Delivered => String::from("delivered"),
            OrderStatus::Cancelled(reason) => format!("cancelled: {}", reason),
        }
    }
}

// --- Option<T> helper ---
fn find_item(items: &[&str], target: &str) -> Option<usize> {
    for (i, item) in items.iter().enumerate() {
        if *item == target {
            return Some(i);
        }
    }
    None
}

// --- Score description using match on ranges ---
fn describe_score(score: u32) -> &'static str {
    match score {
        0 => "no points",
        1..=49 => "low",
        50..=89 => "medium",
        90..=100 => "high",
        _ => "invalid",
    }
}

fn main() {
    // --- Direction ---
    let dir = Direction::North;
    println!("Direction: {:?}, opposite: {:?}", dir, dir.opposite());
    println!("Is horizontal: {}", dir.is_horizontal());

    // --- Message ---
    let messages = vec![
        Message::Quit,
        Message::Echo(String::from("hello")),
        Message::Move { x: 10, y: 20 },
        Message::Color(255, 0, 128),
    ];
    for msg in &messages {
        println!("Message: {}", msg.describe());
    }

    // --- Shape ---
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(4.0, 6.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    for shape in &shapes {
        println!("{}: area = {:.2}", shape.name(), shape.area());
    }

    // --- if let ---
    let msg = Message::Echo(String::from("test"));
    if let Message::Echo(text) = &msg {
        println!("Got echo: {}", text);
    }

    // --- while let ---
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // --- Option ---
    let fruits = ["apple", "banana", "cherry"];
    match find_item(&fruits, "banana") {
        Some(i) => println!("Found banana at index {}", i),
        None => println!("Not found"),
    }

    // --- Score ranges ---
    println!("Score 95: {}", describe_score(95));
    println!("Score 42: {}", describe_score(42));

    // --- Order (enum + struct) ---
    let mut order = Order::new(1, String::from("Rust Book"));
    println!("Order status: {}", order.status_text());
    order.ship(String::from("TRK-999"));
    println!("Order status: {}", order.status_text());
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: Direction opposite
    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
    }

    // Test 2: Direction is_horizontal
    #[test]
    fn test_direction_is_horizontal() {
        assert!(Direction::East.is_horizontal());
        assert!(Direction::West.is_horizontal());
        assert!(!Direction::North.is_horizontal());
        assert!(!Direction::South.is_horizontal());
    }

    // Test 3: Message describe for each variant
    #[test]
    fn test_message_describe() {
        assert_eq!(Message::Quit.describe(), "quit");
        assert_eq!(
            Message::Echo(String::from("hi")).describe(),
            "echo: hi"
        );
        assert_eq!(
            Message::Move { x: 1, y: 2 }.describe(),
            "move to (1, 2)"
        );
        assert_eq!(
            Message::Color(255, 0, 0).describe(),
            "color: rgb(255, 0, 0)"
        );
    }

    // Test 4: Message is_quit
    #[test]
    fn test_message_is_quit() {
        assert!(Message::Quit.is_quit());
        assert!(!Message::Echo(String::from("hi")).is_quit());
    }

    // Test 5: Shape circle area
    #[test]
    fn test_shape_circle_area() {
        let circle = Shape::Circle(5.0);
        let expected = PI * 25.0;
        assert!((circle.area() - expected).abs() < 0.001);
    }

    // Test 6: Shape rectangle area
    #[test]
    fn test_shape_rectangle_area() {
        let rect = Shape::Rectangle(4.0, 6.0);
        assert!((rect.area() - 24.0).abs() < f64::EPSILON);
    }

    // Test 7: Shape triangle area (3-4-5 right triangle)
    #[test]
    fn test_shape_triangle_area() {
        let tri = Shape::Triangle(3.0, 4.0, 5.0);
        assert!((tri.area() - 6.0).abs() < 0.001);
    }

    // Test 8: Shape name
    #[test]
    fn test_shape_name() {
        assert_eq!(Shape::Circle(1.0).name(), "circle");
        assert_eq!(Shape::Rectangle(1.0, 1.0).name(), "rectangle");
        assert_eq!(Shape::Triangle(1.0, 1.0, 1.0).name(), "triangle");
    }

    // Test 9: find_item returns Some when found
    #[test]
    fn test_find_item_found() {
        let items = ["apple", "banana", "cherry"];
        assert_eq!(find_item(&items, "banana"), Some(1));
    }

    // Test 10: find_item returns None when not found
    #[test]
    fn test_find_item_not_found() {
        let items = ["apple", "banana"];
        assert_eq!(find_item(&items, "mango"), None);
    }

    // Test 11: describe_score ranges
    #[test]
    fn test_describe_score() {
        assert_eq!(describe_score(0), "no points");
        assert_eq!(describe_score(25), "low");
        assert_eq!(describe_score(75), "medium");
        assert_eq!(describe_score(95), "high");
        assert_eq!(describe_score(101), "invalid");
    }

    // Test 12: Order status transitions
    #[test]
    fn test_order_status_transitions() {
        let mut order = Order::new(1, String::from("Book"));
        assert_eq!(order.status, OrderStatus::Pending);
        assert_eq!(order.status_text(), "pending");

        order.ship(String::from("TRK-123"));
        assert_eq!(order.status_text(), "shipped (TRK-123)");

        order.deliver();
        assert_eq!(order.status, OrderStatus::Delivered);
        assert_eq!(order.status_text(), "delivered");
    }

    // Test 13: Order cancel
    #[test]
    fn test_order_cancel() {
        let mut order = Order::new(2, String::from("Pen"));
        order.cancel(String::from("out of stock"));
        assert_eq!(order.status_text(), "cancelled: out of stock");
    }

    // Test 14: while let with Option (pop from vec)
    #[test]
    fn test_while_let_pop() {
        let mut stack = vec![10, 20, 30];
        let mut results = Vec::new();
        while let Some(val) = stack.pop() {
            results.push(val);
        }
        assert_eq!(results, vec![30, 20, 10]);
    }

    // Test 15: if let matches correctly
    #[test]
    fn test_if_let_match() {
        let msg = Message::Echo(String::from("test"));
        let mut matched = false;
        if let Message::Echo(text) = &msg {
            assert_eq!(text, "test");
            matched = true;
        }
        assert!(matched);
    }
}
