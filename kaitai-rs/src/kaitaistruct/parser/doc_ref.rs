use crate::kaitaistruct::language::doc_ref::DocRef;
use serde_yaml::Value;
use std::io;

/// Parses the "doc_ref" section of the Kaitai Struct definition.
pub fn parse_doc_ref(doc_ref_instance: &mut DocRef, doc_ref: &Value) -> Result<(), io::Error> {
    match doc_ref {
        Value::String(url) => {
            parse_single_doc_ref(doc_ref_instance, url)?;
        }
        Value::Sequence(urls) => {
            parse_multiple_doc_refs(doc_ref_instance, urls)?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected YAML structure in 'doc_ref' section",
            ));
        }
    }

    Ok(())
}

/// Parses a single URL in the "doc_ref" section.
fn parse_single_doc_ref(doc_ref_instance: &mut DocRef, url: &str) -> Result<(), io::Error> {
    // Add the single URL to the DocRef instance
    doc_ref_instance.add_docref(url)?;

    Ok(())
}

/// Parses multiple URLs in the "doc_ref" section.
fn parse_multiple_doc_refs(
    doc_ref_instance: &mut DocRef,
    urls: &Vec<Value>,
) -> Result<(), io::Error> {
    // Iterate through each URL in the list
    for url_value in urls {
        // Check if each element in the list is a string
        if let Value::String(url) = url_value {
            // Add each URL to the DocRef instance
            doc_ref_instance.add_docref(url)?;
        } else {
            // Handle unexpected data in the list
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid data in 'doc_ref' section",
            ));
        }
    }

    Ok(())
}
