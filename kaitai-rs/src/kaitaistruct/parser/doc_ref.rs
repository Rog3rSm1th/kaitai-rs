use crate::kaitaistruct::language::doc_ref::DocRef;
use serde_yaml::Value;
use std::io;

/// Parses the "doc_ref" section
pub fn parse_doc_ref(doc_ref_instance: &mut DocRef, doc_ref: &Value) -> Result<(), io::Error> {
    match doc_ref {
        // Handle a single string in "doc_ref" section
        Value::String(url) => {
            println!("{}", url);
            doc_ref_instance.add_docref(url).unwrap();
        }
        // Handle a list of strings in "doc_ref" section
        Value::Sequence(urls) => {
            for url_value in urls {
                if let Value::String(url) = url_value {
                    doc_ref_instance.add_docref(url).unwrap();
                } else {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Invalid data in 'doc_ref' section",
                    ));
                }
            }
        }
        _ => {
            // Handle unexpected data in "doc_ref" section
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Unexpected data in 'doc_ref' section",
            ));
        }
    }

    Ok(())
}
