//! # rust_modules
//! `rust_modules` is my studying rust modules.

pub fn item() {}

pub mod aaa {
    pub fn item() {}

    pub mod b {
        pub fn item() {}
    }
}

pub use aaa::b;

pub mod a;

/// Greets to the name given.
///
/// # Examples
///
/// ```
/// let name = "John Doe";
///
/// assert_eq!(rust_modules::greet("John Doe"), "Hello, John Doe!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
