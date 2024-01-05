use crate::kaitaistruct::language::enums::{Enum, Enums};
use crate::kaitaistruct::language::identifier::Identifier;
use serde_yaml::Value;
use std::collections::HashMap;
use std::io;

/// Parses the "enums" section of the Kaitai Struct definition.
pub fn parse_enums(enums_instance: &mut Enums, enums_section: &Value) -> Result<(), io::Error> {
    match enums_section {
        Value::Mapping(enums_map) => {
            // Iterate over each entry in the enums map and parse individual enums
            for (enum_name, enum_values) in enums_map {
                parse_enum(enums_instance, enum_name, enum_values)?;
            }
            Ok(())
        }
        // If "enums" section is not a mapping, return an error
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected YAML structure in enums section",
        )),
    }
}

/// Parses an individual Enum within the "enums" section of the Kaitai Struct definition.
fn parse_enum(
    enums_instance: &mut Enums,
    enum_name: &Value,
    enum_values: &Value,
) -> Result<(), io::Error> {
    // If "enum_values" is a mapping, proceed with parsing
    if let Value::Mapping(variant_map) = enum_values {
        // Create an Identifier for the Enum
        let mut enum_identifier = Identifier::new();

        enum_identifier
            .from_string_vec(vec![enum_name.as_str().unwrap().to_string()])
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        // Create an Enum instance to store parsed variant values
        let mut enum_instance = Enum {
            values: HashMap::new(),
        };

        // Iterate over each variant entry in the Enum
        for (variant_value, variant_name) in variant_map {
            parse_enum_variant(&mut enum_instance, variant_value, variant_name)?;
        }

        // Add the parsed Enum to the Enums instance
        enums_instance.add_enum(enum_identifier, enum_instance)?;
        Ok(())
    } else {
        // If "enum_values" is not a mapping, return an error
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Unexpected YAML structure in enum section",
        ))
    }
}

/// Parses an individual Enum variant within the "enums" section of the Kaitai Struct definition.
fn parse_enum_variant(
    enum_instance: &mut Enum,
    variant_value: &Value,
    variant_name: &Value,
) -> Result<(), io::Error> {
    // If "variant_value" is a number and "variant_name" is a string, proceed with parsing
    if let (Value::Number(value), Value::String(description)) = (variant_value, variant_name) {
        // Extract the variant value as u32
        let variant_value = value.as_u64().ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "Variant value is not a valid u32",
            )
        })? as u32;

        // Insert the variant into the Enum's values HashMap
        enum_instance
            .values
            .insert(variant_value, description.to_string());
        Ok(())
    } else {
        // If the types are not as expected, return an error
        Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Invalid data in enum variant",
        ))
    }
}
