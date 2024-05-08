use crate::ks_language::language::identifier::Identifier;
use std::io;

/// Parses an identifier field
pub fn parse_identifier(
    identifier_instance: &mut Identifier,
    id_str: &str,
) -> Result<(), io::Error> {
    identifier_instance
        .from_string_vec(vec![id_str.to_string()])
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))
}
