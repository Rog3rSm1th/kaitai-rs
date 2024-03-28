use crate::ks_language::language::doc::Doc;
use serde_yaml::Value;
use std::io;

/// Parses the "doc" section of the Kaitai Struct definition.
pub fn parse_doc(doc_instance: &mut Doc, doc: &Value) -> Result<(), io::Error> {
    // Extract the description from the "doc" section
    let description = extract_description(doc)?;

    // Set the description in the Doc instance
    doc_instance.set_description(description);

    Ok(())
}

/// Extracts the description from the "doc" section.
fn extract_description(doc: &Value) -> Result<Option<String>, io::Error> {
    // Check if the "doc" section is a string
    if let Some(description) = doc.as_str() {
        Ok(Some(description.to_string()))
    } else {
        // Handle unexpected data in the "doc" section
        let err_msg = "UnexpectedYamlStructure: doc section".to_string();
        Err(io::Error::new(io::ErrorKind::Other, err_msg))
    }
}
