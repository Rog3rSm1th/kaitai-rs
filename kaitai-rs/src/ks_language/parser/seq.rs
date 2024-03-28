use crate::ks_language::language::seq::Seq;
use crate::ks_language::parser::attribute::parse_attribute;
use serde_yaml::Value;
use std::io;

/// Parses the "seq" section of the Kaitai Struct definition
pub fn parse_seq(seq_instance: &mut Seq, seq: &Value) -> Result<(), io::Error> {
    if let Value::Sequence(sequence) = seq {
        for attribute in sequence {
            let parsed_attribute = parse_attribute(attribute)?;
            seq_instance.add_attribute(parsed_attribute);
        }
    }
    Ok(())
}
