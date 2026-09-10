//! Core greeting library for `rust-template-project`.

use thiserror::Error;

/// Maximum allowed length for a name.
pub const MAX_NAME_LENGTH: usize = 100;

/// Errors that can occur when constructing or using a [`Greeter`].
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum GreetError {
    /// The provided name exceeds the maximum allowed length.
    #[error("the name cannot exceed {max} characters (provided: {actual})")]
    NameTooLong { actual: usize, max: usize },

    /// The name contains prohibited characters such as newlines.
    #[error("the name contains prohibited characters")]
    InvalidCharacters,
}

/// A structure responsible for generating personalized greetings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Greeter {
    name: String,
}

impl Greeter {
    /// Creates a new [`Greeter`] with the provided name after validating it.
    ///
    /// If the provided name is empty or only whitespace, it defaults to `"World"`.
    ///
    /// # Errors
    /// Returns [`GreetError::NameTooLong`] if the name exceeds [`MAX_NAME_LENGTH`] characters.
    /// Returns [`GreetError::InvalidCharacters`] if the name contains newlines.
    ///
    /// # Examples
    /// ```
    /// use rust_template_project::Greeter;
    ///
    /// let greeter = Greeter::new("Ferris").unwrap();
    /// assert_eq!(greeter.name(), "Ferris");
    /// assert_eq!(greeter.greet(), "Hello, Ferris!");
    /// ```
    pub fn new(name: impl Into<String>) -> Result<Self, GreetError> {
        let name = name.into();
        let name = if name.trim().is_empty() {
            "World".to_string()
        } else {
            name
        };

        if name.len() > MAX_NAME_LENGTH {
            tracing::warn!(
                name_len = name.len(),
                max = MAX_NAME_LENGTH,
                "Name exceeds limit"
            );
            return Err(GreetError::NameTooLong {
                actual: name.len(),
                max: MAX_NAME_LENGTH,
            });
        }

        if name.contains('\n') || name.contains('\r') {
            tracing::warn!("Name contains prohibited newline characters");
            return Err(GreetError::InvalidCharacters);
        }

        tracing::debug!(name = %name, "Constructed new Greeter");
        Ok(Self { name })
    }

    /// Returns the name associated with this [`Greeter`].
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Generates a friendly, informal greeting.
    ///
    /// # Examples
    /// ```
    /// use rust_template_project::Greeter;
    ///
    /// let greeter = Greeter::new("Panda").unwrap();
    /// assert_eq!(greeter.greet(), "Hello, Panda!");
    /// ```
    pub fn greet(&self) -> String {
        tracing::debug!(name = %self.name, "Generating informal greeting");
        format!("Hello, {}!", self.name)
    }

    /// Generates a formal greeting.
    ///
    /// # Examples
    /// ```
    /// use rust_template_project::Greeter;
    ///
    /// let greeter = Greeter::new("Panda").unwrap();
    /// assert_eq!(greeter.greet_formal(), "Good day, Panda.");
    /// ```
    pub fn greet_formal(&self) -> String {
        tracing::debug!(name = %self.name, "Generating formal greeting");
        format!("Good day, {}.", self.name)
    }
}

impl Default for Greeter {
    /// Creates a default [`Greeter`] with the name `"World"`.
    fn default() -> Self {
        Self {
            name: "World".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let g = Greeter::new("Panda").unwrap();
        assert_eq!(g.name(), "Panda");
        assert_eq!(g.greet(), "Hello, Panda!");
    }

    #[test]
    fn test_greet_formal() {
        let g = Greeter::new("Panda").unwrap();
        assert_eq!(g.greet_formal(), "Good day, Panda.");
    }

    #[test]
    fn test_greet_empty_name_defaults_to_world() {
        let g = Greeter::new("").unwrap();
        assert_eq!(g.name(), "World");
        assert_eq!(g.greet(), "Hello, World!");
    }

    #[test]
    fn test_greet_whitespace_name_defaults_to_world() {
        let g = Greeter::new("   ").unwrap();
        assert_eq!(g.name(), "World");
        assert_eq!(g.greet(), "Hello, World!");
    }

    #[test]
    fn test_name_too_long_error() {
        let long_name = "a".repeat(MAX_NAME_LENGTH + 1);
        let result = Greeter::new(long_name);
        assert!(matches!(
            result,
            Err(GreetError::NameTooLong {
                actual: 101,
                max: 100
            })
        ));
    }

    #[test]
    fn test_invalid_characters_error() {
        let result = Greeter::new("Panda\nWorld");
        assert_eq!(result, Err(GreetError::InvalidCharacters));
    }

    #[test]
    fn test_default() {
        let g = Greeter::default();
        assert_eq!(g.name(), "World");
        assert_eq!(g.greet(), "Hello, World!");
    }

    #[test]
    fn test_derives() {
        let g1 = Greeter::new("Panda").unwrap();
        let g2 = g1.clone();
        assert_eq!(g1, g2);
        assert_eq!(format!("{g1:?}"), "Greeter { name: \"Panda\" }");
    }
}
