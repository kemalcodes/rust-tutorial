// Rust Tutorial #20: Modules, Crates, and Project Organization
// This is the library root (lib.rs). It declares and re-exports modules.

pub mod modules;

// Re-exports for convenience — users can import directly from the crate root.
pub use modules::math;
pub use modules::models::{Task, TaskStatus, User};
pub use modules::utils;
pub use modules::validation;
