use crate::ks_language::language::identifier::Identifier;
use crate::ks_language::language::instances::Instances;
use crate::ks_language::parser::attribute::parse_attribute;
use crate::ks_language::parser::identifier::parse_identifier;

use serde_yaml::Value;
use std::io;

/// Parses the "instances" section of the Kaitai Struct definition
pub fn parse_instances(
    instances_instance: &mut Instances,
    instances: &Value,
) -> Result<(), io::Error> {
    if let Value::Mapping(instance_map) = instances {
        for (id_value, attr_value) in instance_map {
            let id_str = id_value
                .as_str()
                .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Invalid identifier"))?;
            let mut identifier = Identifier::new();
            parse_identifier(&mut identifier, id_str)?;

            let attribute = parse_attribute(attr_value)?;
            instances_instance.add_attribute(identifier, attribute);
        }
    }
    Ok(())
}
