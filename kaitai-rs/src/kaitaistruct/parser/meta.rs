use crate::kaitaistruct::language::meta::Meta;
use serde_yaml::Value;
use std::io;

/// Parses the "meta" section
pub fn parse_meta(meta_instance: &mut Meta, meta: &Value) -> Result<(), io::Error> {
    Ok(())
}
