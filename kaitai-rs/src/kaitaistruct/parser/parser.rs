use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;
use crate::kaitaistruct::language::enums::Enums;
use crate::kaitaistruct::language::meta::Meta;
use crate::kaitaistruct::language::params::Params;
use crate::kaitaistruct::language::types::Types;
use crate::kaitaistruct::parser::doc::parse_doc;
use crate::kaitaistruct::parser::doc_ref::parse_doc_ref;
use crate::kaitaistruct::parser::enums::parse_enums;
use crate::kaitaistruct::parser::seq::parse_seq;
use crate::kaitaistruct::parser::meta::parse_meta;
use crate::kaitaistruct::parser::params::parse_params;
use crate::kaitaistruct::parser::types::parse_types;
use crate::kaitaistruct::language::seq::Seq;
use serde_yaml::Value;
use std::fs::File;
use std::io::{self, Read};

// KsyParser struct to handle parsing logic
pub struct KsyParser {
    pub doc: Doc,
    pub doc_ref: DocRef,
    pub enums: Enums,
    pub meta: Meta,
    pub params: Params,
    pub seq: Seq,
    pub types: Types,
}

impl KsyParser {
    pub fn new() -> Self {
        KsyParser {
            doc: Doc::new(),
            doc_ref: DocRef::new(),
            enums: Enums::new(),
            meta: Meta::new(),
            params: Params::new(),
            seq: Seq::new(),
            types: Types::new(),
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
                    parse_meta(&mut self.meta, meta).unwrap();
                }

                // Process the "doc" section
                if let Some(doc) = map.get(&Value::String("doc".to_string())) {
                    parse_doc(&mut self.doc, doc).unwrap();
                }

                // Process the "doc_ref" section
                if let Some(doc_ref) = map.get(&Value::String("doc-ref".to_string())) {
                    parse_doc_ref(&mut self.doc_ref, doc_ref).unwrap();
                }

                // Process the "params" section
                if let Some(params) = map.get(&Value::String("params".to_string())) {
                    parse_params(&mut self.params, params).unwrap();
                }

                // Process the "seq" section
                if let Some(seq) = map.get(&Value::String("seq".to_string())) {
                    parse_seq(&mut self.seq, seq).unwrap();
                }

                // Process the "types" section
                if let Some(types) = map.get(&Value::String("types".to_string())) {
                    parse_types(&mut self.types, types).unwrap();
                }

                // Process the "instances" section
                if let Some(_instances) = map.get(&Value::String("instances".to_string())) {
                    // TODO: Implement processing for "instances"
                }

                // Process the "enums" section
                if let Some(enums) = map.get(&Value::String("enums".to_string())) {
                    parse_enums(&mut self.enums, enums).unwrap();
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
        println!("{:#?}", self.doc);
        println!("{:#?}", self.doc_ref);
        println!("{:#?}", self.enums);
        println!("{:#?}", self.meta);
        println!("{:#?}", self.params);
        println!("{:#?}", self.seq);
    }
}
