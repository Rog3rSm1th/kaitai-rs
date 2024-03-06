use crate::kaitaistruct::language::attribute::Attribute;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::language::seq::Seq;
use crate::kaitaistruct::parser::identifier::parse_identifier;
use serde_yaml::Value;
use std::io;

pub fn parse_attribute(
    seq_instance: &mut Seq,
    attribute: &Value,
) -> Result<(), io::Error> {
    // Parse the "id" field as the identifier
    let id_str = attribute
        .get("id")
        .and_then(|value| value.as_str())
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "Missing or invalid 'id' field"))?;

    let mut identifier = Identifier::new();
    parse_identifier(&mut identifier, id_str)?;

    // Create a new Attribute instance with default values
    let new_attribute = Attribute::new(
        identifier,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    );

    // Add the new Attribute instance to the sequence
    seq_instance.add_attribute(new_attribute);

    Ok(())
}