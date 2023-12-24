use crate::config::config::Config;
use crate::errors::KaitaiError;
use crate::utils::utils::validate_values;
use crate::kaitaistruct::language::identifier::Identifier;
use std::collections::HashMap;
use std::io;

// Enums property struct
#[derive(Debug)]
pub struct Enums {
    // Hashmap of the existing enums
    pub enums_specs: HashMap<Identifier, Enum>,
}

impl Enums {
    pub fn new() -> Self {
        Enums {
            enums_specs: HashMap::new(),
        }
    }

    // Adds an Enum to the Enums instance
    pub fn add_enum(&mut self, identifier: Identifier, enum_instance: Enum) -> Result<(), io::Error> {
        self.enums_specs.insert(identifier, enum_instance);
        Ok(())
    }
}

// Enum struct definition
#[derive(Debug)]
pub struct Enum {
    // Enum possible values
    pub values: HashMap<u32, String>,
}

impl Enum {
    pub fn new(values: HashMap<u32, String>) -> Result<Self, KaitaiError> {
        Ok(Enum { values })
    }
}