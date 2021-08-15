pub fn item() {}

pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(greet("John Doe"), "Hello, John Doe!");
    }
}
