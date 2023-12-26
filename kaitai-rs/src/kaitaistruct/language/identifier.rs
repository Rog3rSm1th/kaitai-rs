use crate::config::config::Config;
use crate::utils::utils::validate_values;
use std::io;

// Identifier struct to represent an identifier
#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Identifier {
    // Vector containing identifier values
    value: Vec<String>,
}

impl Identifier {
    pub fn new(values: Vec<String>) -> Result<Self, io::Error> {
        // Check if all values match the identifier pattern
        validate_values(&values, Config::IDENTIFIER_PATTERN);

        Ok(Identifier { value: values })
    }

    // Getter method to retrieve identifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }

    // Create an Identifier instance from a string
    pub fn from_string(identifier: &str) -> Result<Self, io::Error> {
        let values: Vec<String> = identifier.split('.').map(String::from).collect();
        Identifier::new(values)
    }
}
