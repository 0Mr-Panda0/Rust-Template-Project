pub struct Greeter {
    name: String,
}

impl Greeter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }

    pub fn greet_formal(&self) -> String {
        format!("Good day, {}.", self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let g = Greeter::new("Panda");
        assert_eq!(g.greet(), "Hello, Panda!");
    }

    #[test]
    fn test_greet_formal() {
        let g = Greeter::new("Panda");
        assert_eq!(g.greet_formal(), "Good day, Panda.");
    }

    #[test]
    fn test_greet_empty_name() {
        let g = Greeter::new("");
        assert_eq!(g.greet(), "Hello, !");
    }
}
