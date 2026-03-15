// Rust Tutorial #6: Structs and Methods
// Demonstrates struct definitions, impl blocks, methods (&self, &mut self),
// associated functions (::new), tuple structs, Display, and derive macros.

use std::fmt;

// --- Basic Struct ---
#[derive(Debug, Clone, PartialEq)]
struct User {
    name: String,
    email: String,
    age: u32,
    active: bool,
}

impl User {
    fn new(name: String, email: String, age: u32) -> User {
        User {
            name,
            email,
            age,
            active: true,
        }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }

    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    fn summary(&self) -> String {
        format!("{} ({}) — age {}", self.name, self.email, self.age)
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let status = if self.active { "active" } else { "inactive" };
        write!(f, "{} <{}> [{}]", self.name, self.email, status)
    }
}

// --- Struct with Methods ---
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }

    fn square(size: f64) -> Rectangle {
        Rectangle { width: size, height: size }
    }

    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    fn scale(&mut self, factor: f64) {
        self.width *= factor;
        self.height *= factor;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rectangle({}x{})", self.width, self.height)
    }
}

// --- Tuple Struct ---
#[derive(Debug, Clone, PartialEq)]
struct Meters(f64);

impl Meters {
    fn to_feet(&self) -> f64 {
        self.0 * 3.28084
    }
}

// --- Bank Account Example ---
#[derive(Debug, Clone)]
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    fn new(owner: String, initial_balance: f64) -> BankAccount {
        BankAccount {
            owner,
            balance: initial_balance,
        }
    }

    fn deposit(&mut self, amount: f64) {
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
            true
        } else {
            false
        }
    }

    fn balance(&self) -> f64 {
        self.balance
    }
}

impl fmt::Display for BankAccount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Account({}, ${:.2})", self.owner, self.balance)
    }
}

fn main() {
    // --- User struct ---
    let mut user = User::new(
        String::from("Alex"),
        String::from("alex@example.com"),
        28,
    );
    println!("User: {}", user);
    println!("Summary: {}", user.summary());
    println!("Is adult: {}", user.is_adult());

    user.deactivate();
    println!("After deactivate: {}", user);

    // --- Rectangle ---
    let mut rect = Rectangle::new(10.0, 5.0);
    println!("\n{}", rect);
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());

    rect.scale(2.0);
    println!("After scale(2): {}", rect);

    let sq = Rectangle::square(7.0);
    println!("Square: {} — is_square: {}", sq, sq.is_square());

    // --- Tuple struct ---
    let distance = Meters(100.0);
    println!("\n{:?} = {:.1} feet", distance, distance.to_feet());

    // --- Bank Account ---
    let mut account = BankAccount::new(String::from("Sam"), 1000.0);
    println!("\n{}", account);
    account.deposit(500.0);
    println!("After deposit: {}", account);
    let ok = account.withdraw(200.0);
    println!("Withdraw 200 success: {} — {}", ok, account);

    // --- Clone and PartialEq ---
    let r1 = Rectangle::new(3.0, 4.0);
    let r2 = r1.clone();
    println!("\nr1 == r2: {}", r1 == r2);
}

// --- Unit Tests ---
#[cfg(test)]
mod tests {
    use super::*;

    // Test 1: User::new creates user with correct fields
    #[test]
    fn test_user_new() {
        let user = User::new(String::from("Alex"), String::from("a@b.com"), 25);
        assert_eq!(user.name, "Alex");
        assert_eq!(user.email, "a@b.com");
        assert_eq!(user.age, 25);
        assert!(user.active);
    }

    // Test 2: User deactivate sets active to false
    #[test]
    fn test_user_deactivate() {
        let mut user = User::new(String::from("Sam"), String::from("s@b.com"), 30);
        user.deactivate();
        assert!(!user.active);
    }

    // Test 3: User is_adult
    #[test]
    fn test_user_is_adult() {
        let adult = User::new(String::from("Alex"), String::from("a@b.com"), 18);
        let child = User::new(String::from("Jo"), String::from("j@b.com"), 12);
        assert!(adult.is_adult());
        assert!(!child.is_adult());
    }

    // Test 4: Rectangle area
    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10.0, 5.0);
        assert!((rect.area() - 50.0).abs() < f64::EPSILON);
    }

    // Test 5: Rectangle perimeter
    #[test]
    fn test_rectangle_perimeter() {
        let rect = Rectangle::new(10.0, 5.0);
        assert!((rect.perimeter() - 30.0).abs() < f64::EPSILON);
    }

    // Test 6: Rectangle::square creates a square
    #[test]
    fn test_rectangle_square() {
        let sq = Rectangle::square(7.0);
        assert!(sq.is_square());
        assert!((sq.area() - 49.0).abs() < f64::EPSILON);
    }

    // Test 7: Rectangle scale mutates dimensions
    #[test]
    fn test_rectangle_scale() {
        let mut rect = Rectangle::new(4.0, 3.0);
        rect.scale(2.0);
        assert!((rect.width - 8.0).abs() < f64::EPSILON);
        assert!((rect.height - 6.0).abs() < f64::EPSILON);
    }

    // Test 8: Rectangle can_hold
    #[test]
    fn test_rectangle_can_hold() {
        let big = Rectangle::new(10.0, 8.0);
        let small = Rectangle::new(5.0, 4.0);
        assert!(big.can_hold(&small));
        assert!(!small.can_hold(&big));
    }

    // Test 9: Tuple struct Meters to_feet conversion
    #[test]
    fn test_meters_to_feet() {
        let m = Meters(1.0);
        assert!((m.to_feet() - 3.28084).abs() < 0.001);
    }

    // Test 10: BankAccount deposit and withdraw
    #[test]
    fn test_bank_account_operations() {
        let mut acc = BankAccount::new(String::from("Jordan"), 100.0);
        acc.deposit(50.0);
        assert!((acc.balance() - 150.0).abs() < f64::EPSILON);

        let ok = acc.withdraw(30.0);
        assert!(ok);
        assert!((acc.balance() - 120.0).abs() < f64::EPSILON);
    }

    // Test 11: BankAccount withdraw fails when insufficient funds
    #[test]
    fn test_bank_account_withdraw_fails() {
        let mut acc = BankAccount::new(String::from("Sam"), 50.0);
        let ok = acc.withdraw(100.0);
        assert!(!ok);
        assert!((acc.balance() - 50.0).abs() < f64::EPSILON);
    }

    // Test 12: BankAccount negative deposit is ignored
    #[test]
    fn test_bank_account_negative_deposit() {
        let mut acc = BankAccount::new(String::from("Alex"), 100.0);
        acc.deposit(-50.0);
        assert!((acc.balance() - 100.0).abs() < f64::EPSILON);
    }

    // Test 13: Clone creates independent copy
    #[test]
    fn test_rectangle_clone_independence() {
        let r1 = Rectangle::new(5.0, 3.0);
        let mut r2 = r1.clone();
        r2.scale(2.0);
        assert!((r1.width - 5.0).abs() < f64::EPSILON);
        assert!((r2.width - 10.0).abs() < f64::EPSILON);
    }

    // Test 14: PartialEq works for structs
    #[test]
    fn test_rectangle_equality() {
        let r1 = Rectangle::new(3.0, 4.0);
        let r2 = Rectangle::new(3.0, 4.0);
        let r3 = Rectangle::new(4.0, 3.0);
        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
    }

    // Test 15: Display trait works
    #[test]
    fn test_user_display() {
        let user = User::new(String::from("Alex"), String::from("a@b.com"), 25);
        let display = format!("{}", user);
        assert!(display.contains("Alex"));
        assert!(display.contains("a@b.com"));
        assert!(display.contains("active"));
    }
}
