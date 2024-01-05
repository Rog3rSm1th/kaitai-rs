use crate::config::Config;
use crate::utils::validate_values;
use std::io;

// Identifier struct to represent an identifier
#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Identifier {
    // Vector containing identifier values
    value: Vec<String>,
}

impl Identifier {
    pub fn new() -> Self {
        Identifier { value: Vec::new() }
    }

    // Getter method to retrieve identifier values
    #[allow(dead_code)]
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }

    // Sets the value field instance from a string
    pub fn from_string(&mut self, identifier: &str) -> Result<(), io::Error> {
        let values: Vec<String> = vec![identifier.to_string()];

        // Check if all values match the identifier pattern
        validate_values(&values, Config::IDENTIFIER_PATTERN).unwrap();
        self.value = values;

        Ok(())
    }

    // Sets the value field from a vector of strings
    pub fn from_string_vec(&mut self, identifiers: Vec<String>) -> Result<(), io::Error> {
        let values: Vec<String> = identifiers;

        // Check if all values match the identifier pattern
        validate_values(&values, Config::IDENTIFIER_PATTERN).unwrap();
        self.value = values;

        Ok(())
    }
}
