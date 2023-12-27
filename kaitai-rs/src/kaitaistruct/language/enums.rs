use crate::kaitaistruct::language::identifier::Identifier;
use std::collections::HashMap;
use std::io;

/// Structure representing a collection of Enums in a Kaitai Struct.
#[derive(Debug)]
pub struct Enums {
    /// HashMap containing the specifications for each Enum.
    pub enums_specs: HashMap<Identifier, Enum>,
}

impl Enums {
    /// Creates a new instance of `Enums` with an empty HashMap.
    pub fn new() -> Self {
        Enums {
            enums_specs: HashMap::new(),
        }
    }

    /// Adds an Enum to the Enums instance.
    pub fn add_enum(
        &mut self,
        identifier: Identifier,
        enum_instance: Enum,
    ) -> Result<(), io::Error> {
        self.enums_specs.insert(identifier, enum_instance);
        Ok(())
    }
}

/// Structure representing an Enum in a Kaitai Struct.
#[derive(Debug)]
pub struct Enum {
    /// HashMap containing the possible values of the Enum.
    pub values: HashMap<u32, String>,
}

impl Enum {
    /// Creates a new instance of `Enum` with the specified values.
    pub fn new(values: HashMap<u32, String>) -> Result<Self, io::Error> {
        Ok(Enum { values })
    }
}
