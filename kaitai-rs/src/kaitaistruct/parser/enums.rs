use crate::kaitaistruct::language::enums::Enum;
use crate::kaitaistruct::language::enums::Enums;
use crate::kaitaistruct::language::identifier::Identifier;
use serde_yaml::Value;
use std::collections::HashMap;
use std::io;

/// Parses the "enums" section
pub fn parse_enums(enums_instance: &mut Enums, enums_section: &Value) -> Result<(), io::Error> {
    match enums_section {
        Value::Mapping(enums_map) => {
            for (enum_name, enum_values) in enums_map {
                if let Value::Mapping(variant_map) = enum_values {
                    // Create an Identifier for the Enum
                    let enum_identifier =
                        Identifier::new(vec![enum_name.as_str().unwrap().to_string()]).map_err(
                            |e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()),
                        )?;

                    // Create an Enum instance
                    let mut enum_instance = Enum {
                        values: HashMap::new(),
                    };

                    for (variant_value, variant_name) in variant_map {
                        if let (Value::Number(value), Value::String(description)) =
                            (variant_value, variant_name)
                        {
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
                        } else {
                            // Handle unexpected data in enum variant
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                "Unexpected data in enum variant",
                            ));
                        }
                    }

                    // Validate the enum name and add it to Enums
                    enums_instance.add_enum(enum_identifier, enum_instance)?;
                } else {
                    // Handle unexpected data in enum section
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Unexpected data in enum section",
                    ));
                }
            }

            // Return Ok since Enums instance is modified in place
            Ok(())
        }
        _ => {
            // Handle unexpected YAML structure
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected YAML structure in enums section",
            ))
        }
    }
}
