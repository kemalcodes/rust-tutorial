// Data models module
// Demonstrates structs, enums, impl blocks, and Display trait in a module.

use std::fmt;

/// Represents a user in the system.
#[derive(Debug, Clone, PartialEq)]
pub struct User {
    pub name: String,
    pub email: String,
    age: u32, // Private field — not accessible outside this module
}

impl User {
    /// Creates a new User.
    pub fn new(name: &str, email: &str, age: u32) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            age,
        }
    }

    /// Returns the user's age. This is the only way to access the private field.
    pub fn age(&self) -> u32 {
        self.age
    }

    /// Checks if the user is an adult (18 or older).
    pub fn is_adult(&self) -> bool {
        self.age >= 18
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

/// Status of a task.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
            TaskStatus::Cancelled => write!(f, "Cancelled"),
        }
    }
}

/// A task with a title and status.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub title: String,
    pub status: TaskStatus,
    pub assignee: Option<User>,
}

impl Task {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            status: TaskStatus::Pending,
            assignee: None,
        }
    }

    pub fn assign_to(&mut self, user: User) {
        self.assignee = Some(user);
        self.status = TaskStatus::InProgress;
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
    }

    pub fn is_done(&self) -> bool {
        self.status == TaskStatus::Completed
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.status, self.title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User::new("Alex", "alex@example.com", 25);
        assert_eq!(user.name, "Alex");
        assert_eq!(user.email, "alex@example.com");
        assert_eq!(user.age(), 25);
    }

    #[test]
    fn test_user_is_adult() {
        assert!(User::new("Alex", "a@b.com", 18).is_adult());
        assert!(!User::new("Sam", "s@b.com", 17).is_adult());
    }

    #[test]
    fn test_user_display() {
        let user = User::new("Alex", "alex@example.com", 25);
        assert_eq!(format!("{}", user), "Alex <alex@example.com>");
    }

    #[test]
    fn test_task_lifecycle() {
        let mut task = Task::new("Write tests");
        assert_eq!(task.status, TaskStatus::Pending);
        assert!(!task.is_done());

        let user = User::new("Alex", "alex@example.com", 25);
        task.assign_to(user);
        assert_eq!(task.status, TaskStatus::InProgress);

        task.complete();
        assert!(task.is_done());
    }

    #[test]
    fn test_task_display() {
        let task = Task::new("Deploy app");
        assert_eq!(format!("{}", task), "[Pending] Deploy app");
    }

    #[test]
    fn test_task_status_display() {
        assert_eq!(format!("{}", TaskStatus::InProgress), "In Progress");
        assert_eq!(format!("{}", TaskStatus::Cancelled), "Cancelled");
    }
}
