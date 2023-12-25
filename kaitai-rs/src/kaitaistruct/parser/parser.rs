use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enum;
use crate::kaitaistruct::language::enums::Enums;
use crate::kaitaistruct::language::identifier::Identifier;
use crate::kaitaistruct::parser::doc::parse_doc;
use crate::kaitaistruct::parser::doc_ref::parse_doc_ref;
use crate::kaitaistruct::parser::enums::parse_enums;
use serde_yaml::Value;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read};

// KsyParser struct to handle parsing logic
pub struct KsyParser {
    pub doc_instance: Doc,
    pub doc_ref_instance: DocRef,
    pub enums_instance: Enums,
}

impl KsyParser {
    pub fn new() -> Self {
        KsyParser {
            doc_instance: Doc::new(),
            doc_ref_instance: DocRef::new(),
            enums_instance: Enums::new(),
        }
    }

    pub fn parse_yaml(&mut self, file_path: &str) -> Result<Value, io::Error> {
        // Open the file
        let file = File::open(file_path)?;

        // Create a buffered reader to efficiently read the file
        let mut reader = io::BufReader::new(file);

        // Read the file content into a String
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        // Parse the YAML content into serde_yaml::Value
        let yaml_value: Value = serde_yaml::from_str(&content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

        self.parse_sections(&yaml_value)?;

        Ok(yaml_value)
    }

    /// Parses and processes sections from a Kaitai Struct YAML file.
    pub fn parse_sections(&mut self, yaml_value: &Value) -> Result<(), io::Error> {
        // Match and process each section
        match yaml_value {
            Value::Mapping(map) => {
                // Process the "meta" section
                if let Some(meta) = map.get(&Value::String("meta".to_string())) {
                    // TODO: Implement processing for "meta"
                }

                // Process the "doc" section
                if let Some(doc) = map.get(&Value::String("doc".to_string())) {
                    parse_doc(&mut self.doc_instance, doc)?;
                }

                // Process the "doc_ref" section
                if let Some(doc_ref) = map.get(&Value::String("doc-ref".to_string())) {
                    parse_doc_ref(&mut self.doc_ref_instance, doc_ref)?;
                }

                // Process the "params" section
                if let Some(params) = map.get(&Value::String("params".to_string())) {
                    // TODO: Implement processing for "params"
                }

                // Process the "seq" section
                if let Some(seq) = map.get(&Value::String("seq".to_string())) {
                    // TODO: Implement processing for "seq"
                }

                // Process the "types" section
                if let Some(types) = map.get(&Value::String("types".to_string())) {
                    // TODO: Implement processing for "types"
                }

                // Process the "instances" section
                if let Some(instances) = map.get(&Value::String("instances".to_string())) {
                    // TODO: Implement processing for "instances"
                }

                // Process the "enums" section
                if let Some(enums) = map.get(&Value::String("enums".to_string())) {
                    parse_enums(&mut self.enums_instance, enums)?;
                }
            }
            _ => {
                // Handle unexpected YAML structure
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "Unexpected YAML structure",
                ));
            }
        }

        Ok(())
    }

    /// Prints the Ksy sections
    pub fn print_struct(&self) {
        println!("{:#?}", self.doc_instance);
        println!("{:#?}", self.doc_ref_instance);
        println!("{:#?}", self.enums_instance);
    }
}
