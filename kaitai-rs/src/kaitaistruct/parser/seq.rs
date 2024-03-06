use crate::kaitaistruct::language::seq::Seq;
use crate::kaitaistruct::parser::attribute::parse_attribute;
use serde_yaml::Value;
use std::io;

/// Parses the "seq" section of the Kaitai Struct definition
pub fn parse_seq(
    seq_instance: &mut Seq,
    seq: &Value,
) -> Result<(), io::Error> {
    if let Value::Sequence(sequence) = seq {
        for attribute in sequence {
            parse_attribute(seq_instance, attribute)?;
        }
    } 
    Ok(())
}