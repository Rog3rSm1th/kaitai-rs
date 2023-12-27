use crate::kaitaistruct::language::meta::Meta;
use serde_yaml::Value;
use std::io;

/// Parses the "meta" section
pub fn parse_meta(_meta_instance: &mut Meta, _meta: &Value) -> Result<(), io::Error> {
    Ok(())
}
