use crate::config::config::Config;
use crate::errors::KaitaiError;
use crate::kaitaistruct::kaitai_property::KaitaiProperty;
use crate::utils::utils::validate_values;
use regex::Regex;
use std::collections::HashMap;

/// Enums property struct
pub struct Enums {
    // List of the existing enums
    pub enums: Vec<Enum>,
}

impl KaitaiProperty for Enums {}

// Enum struct
pub struct Enum {
    // Name of the enum
    pub name: String,
    // Enum possible values
    pub values: HashMap<String, String>,
}

impl Enum {
    pub fn new(name: String, values: HashMap<String, String>) -> Result<Self, KaitaiError> {
        // Check if all values match the identifier pattern
        validate_values(
            &[name.clone()],
            Config::ENUM_NAME_PATTERN,
            KaitaiError::BadEnumName,
        )?;

        Ok(Enum { name, values })
    }
}
