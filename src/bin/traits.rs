// Rust Tutorial #9: Traits — Shared Behavior
// Demonstrates defining traits, implementing traits, default methods,
// trait bounds, derive macros, Display trait, operator overloading, and trait objects.

use std::fmt;
use std::ops::Add;

// --- Defining a trait ---

trait Describable {
    fn describe(&self) -> String;

    // Default method — implementations can override this
    fn summary(&self) -> String {
        format!("Summary: {}", self.describe())
    }
}

// --- Implementing traits for different types ---

#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    age: u32,
}

impl Describable for User {
    fn describe(&self) -> String {
        format!("{} (age {})", self.name, self.age)
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Product {
    name: String,
    price: f64,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("{} — ${:.2}", self.name, self.price)
    }

    // Override the default method
    fn summary(&self) -> String {
        format!("Product: {} costs ${:.2}", self.name, self.price)
    }
}

// --- Display trait (custom printing) ---

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (age {})", self.name, self.age)
    }
}

impl fmt::Display for Product {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} — ${:.2}", self.name, self.price)
    }
}

// --- Operator overloading ---

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// --- Trait bounds ---

fn print_description<T: Describable>(item: &T) {
    println!("{}", item.describe());
}

fn print_two_descriptions<T: Describable + fmt::Display>(item: &T) {
    println!("Description: {}", item.describe());
    println!("Display: {}", item);
}

// Where clause — same as trait bounds but cleaner for complex signatures
fn longest_description<T>(a: &T, b: &T) -> String
where
    T: Describable,
{
    let desc_a = a.describe();
    let desc_b = b.describe();
    if desc_a.len() >= desc_b.len() {
        desc_a
    } else {
        desc_b
    }
}

// --- Trait objects (dyn) ---

fn describe_all(items: &[&dyn Describable]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}

// Return a trait object
fn create_describable(use_user: bool) -> Box<dyn Describable> {
    if use_user {
        Box::new(User {
            name: String::from("Alex"),
            age: 30,
        })
    } else {
        Box::new(Product {
            name: String::from("Laptop"),
            price: 999.99,
        })
    }
}

// --- Multiple traits on one type ---

trait Printable {
    fn print_info(&self) -> String;
}

impl Printable for User {
    fn print_info(&self) -> String {
        format!("User: {}", self.name)
    }
}

impl Printable for Product {
    fn print_info(&self) -> String {
        format!("Product: {}", self.name)
    }
}

// --- Derive macros demonstration ---

#[derive(Debug, Clone, PartialEq)]
struct Task {
    title: String,
    done: bool,
}

fn main() {
    // Basic trait usage
    let user = User {
        name: String::from("Alex"),
        age: 30,
    };
    let product = Product {
        name: String::from("Laptop"),
        price: 999.99,
    };

    println!("=== Trait Methods ===");
    println!("{}", user.describe());
    println!("{}", product.describe());
    println!("{}", user.summary()); // Uses default
    println!("{}", product.summary()); // Uses override

    println!("\n=== Display Trait ===");
    println!("User: {}", user);
    println!("Product: {}", product);

    println!("\n=== Operator Overloading ===");
    let p1 = Point { x: 1.0, y: 2.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    let p3 = p1 + p2;
    println!("{} + {} = {}", p1, p2, p3);

    println!("\n=== Trait Bounds ===");
    print_description(&user);
    print_description(&product);
    print_two_descriptions(&user);

    println!("\n=== Where Clause ===");
    let user2 = User {
        name: String::from("Sam"),
        age: 25,
    };
    let longest = longest_description(&user, &user2);
    println!("Longest: {}", longest);

    println!("\n=== Trait Objects ===");
    let items: Vec<&dyn Describable> = vec![&user, &product];
    let descriptions = describe_all(&items);
    for desc in &descriptions {
        println!("  {}", desc);
    }

    let boxed = create_describable(true);
    println!("Boxed: {}", boxed.describe());

    println!("\n=== Multiple Traits ===");
    println!("{}", user.print_info());
    println!("{}", product.print_info());

    println!("\n=== Derive Macros ===");
    let task1 = Task {
        title: String::from("Write code"),
        done: false,
    };
    let task2 = task1.clone();
    println!("Debug: {:?}", task1);
    println!("Equal: {}", task1 == task2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_describe() {
        let user = User {
            name: String::from("Alex"),
            age: 30,
        };
        assert_eq!(user.describe(), "Alex (age 30)");
    }

    #[test]
    fn test_user_default_summary() {
        let user = User {
            name: String::from("Alex"),
            age: 30,
        };
        assert_eq!(user.summary(), "Summary: Alex (age 30)");
    }

    #[test]
    fn test_product_describe() {
        let product = Product {
            name: String::from("Laptop"),
            price: 999.99,
        };
        assert_eq!(product.describe(), "Laptop — $999.99");
    }

    #[test]
    fn test_product_overridden_summary() {
        let product = Product {
            name: String::from("Laptop"),
            price: 999.99,
        };
        assert_eq!(product.summary(), "Product: Laptop costs $999.99");
    }

    #[test]
    fn test_display_user() {
        let user = User {
            name: String::from("Sam"),
            age: 25,
        };
        assert_eq!(format!("{}", user), "Sam (age 25)");
    }

    #[test]
    fn test_display_product() {
        let product = Product {
            name: String::from("Book"),
            price: 19.99,
        };
        assert_eq!(format!("{}", product), "Book — $19.99");
    }

    #[test]
    fn test_point_add() {
        let p1 = Point { x: 1.0, y: 2.0 };
        let p2 = Point { x: 3.0, y: 4.0 };
        let result = p1 + p2;
        assert_eq!(result, Point { x: 4.0, y: 6.0 });
    }

    #[test]
    fn test_display_point() {
        let p = Point { x: 1.5, y: 2.5 };
        assert_eq!(format!("{}", p), "(1.5, 2.5)");
    }

    #[test]
    fn test_longest_description() {
        let user1 = User {
            name: String::from("Alex"),
            age: 30,
        };
        let user2 = User {
            name: String::from("Sam"),
            age: 25,
        };
        let result = longest_description(&user1, &user2);
        assert_eq!(result, "Alex (age 30)");
    }

    #[test]
    fn test_describe_all() {
        let user = User {
            name: String::from("Alex"),
            age: 30,
        };
        let product = Product {
            name: String::from("Book"),
            price: 9.99,
        };
        let items: Vec<&dyn Describable> = vec![&user, &product];
        let descriptions = describe_all(&items);
        assert_eq!(descriptions.len(), 2);
        assert_eq!(descriptions[0], "Alex (age 30)");
        assert_eq!(descriptions[1], "Book — $9.99");
    }

    #[test]
    fn test_create_describable_user() {
        let item = create_describable(true);
        assert_eq!(item.describe(), "Alex (age 30)");
    }

    #[test]
    fn test_create_describable_product() {
        let item = create_describable(false);
        assert_eq!(item.describe(), "Laptop — $999.99");
    }

    #[test]
    fn test_derive_clone() {
        let task = Task {
            title: String::from("Test"),
            done: false,
        };
        let cloned = task.clone();
        assert_eq!(task, cloned);
    }

    #[test]
    fn test_derive_debug() {
        let task = Task {
            title: String::from("Test"),
            done: true,
        };
        let debug_str = format!("{:?}", task);
        assert!(debug_str.contains("Test"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_multiple_traits() {
        let user = User {
            name: String::from("Alex"),
            age: 30,
        };
        assert_eq!(user.print_info(), "User: Alex");
        assert_eq!(user.describe(), "Alex (age 30)");

        let product = Product {
            name: String::from("Book"),
            price: 9.99,
        };
        assert_eq!(product.print_info(), "Product: Book");
    }

    #[test]
    fn test_derive_partial_eq() {
        let user1 = User {
            name: String::from("Alex"),
            age: 30,
        };
        let user2 = User {
            name: String::from("Alex"),
            age: 30,
        };
        let user3 = User {
            name: String::from("Sam"),
            age: 25,
        };
        assert_eq!(user1, user2);
        assert_ne!(user1, user3);
    }
}
