pub fn item() {}

pub mod aaa {
    pub fn item() {}
}

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
