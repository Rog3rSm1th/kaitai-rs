use crate::kaitaistruct::language::doc::Doc;
use serde_yaml::Value;
use std::io;

/// Parses the "doc" section
pub fn parse_doc(doc_instance: &mut Doc, doc: &Value) -> Result<(), io::Error> {
    // Extract the description from the "doc" section
    let description = doc.as_str().map(|s| s.to_string());

    // Set the description in the Doc instance
    doc_instance.set_description(description);

    Ok(())
}
