use crate::config::config::Config;
use crate::errors::KaitaiError;
use crate::utils::utils::validate_values;

// Identifier struct to represent an identifier
pub struct Identifier {
    // Vector containing identifier values
    value: Vec<String>,
}

impl Identifier {
    fn new(values: Vec<String>) -> Result<Self, KaitaiError> {
        // Check if all values match the identifier pattern
        validate_values(
            &values,
            Config::IDENTIFIER_PATTERN,
            KaitaiError::BadMetaIdentifier,
        )?;

        Ok(Identifier { value: values })
    }

    // Getter method to retrieve identifier values
    fn get_values(&self) -> &Vec<String> {
        &self.value
    }
}
