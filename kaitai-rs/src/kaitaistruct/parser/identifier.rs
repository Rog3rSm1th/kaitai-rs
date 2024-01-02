use crate::kaitaistruct::language::identifier::Identifier;
use std::io;

/// Parses the identifier field within the "meta" section
pub fn parse_identifier(id_str: &str) -> Result<Identifier, io::Error> {
    Identifier::new(vec![id_str.to_string()])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}
