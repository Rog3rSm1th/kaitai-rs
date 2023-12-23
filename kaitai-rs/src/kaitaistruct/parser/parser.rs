use serde_yaml::Value;
use std::fs::File;
use std::io::{self, Read};
use crate::kaitaistruct::language::doc::Doc;
use crate::kaitaistruct::language::doc_ref::DocRef;

// KsyParser struct to handle parsing logic
pub struct KsyParser {
    pub doc_instance: Doc,
    pub doc_ref_instance: DocRef,
}

impl KsyParser {
    pub fn new() -> Self {
        KsyParser {
            doc_instance: Doc::new(),
            doc_ref_instance: DocRef::new(),
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
                    self.parse_doc(doc)?;
                }

                // Process the "doc_ref" section
                if let Some(doc_ref) = map.get(&Value::String("doc-ref".to_string())) {
                    self.parse_doc_ref(doc_ref)?;
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
                    // TODO: Implement processing for "enums"
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

    /// Parses the "doc" section and returns a Doc instance.
    fn parse_doc(&mut self, doc: &Value) -> Result<(), io::Error> {
        // Extract the description from the "doc" section
        let description = doc.as_str().map(|s| s.to_string());

        // Set the description in the Doc instance
        self.doc_instance.set_description(description);

        Ok(())
    }

    /// Parses the "doc_ref" section and returns a DocRef instance.
    fn parse_doc_ref(&mut self, doc_ref: &Value) -> Result<(), io::Error> {
        match doc_ref {
            // Handle a single string in "doc_ref" section
            Value::String(url) => {
                println!("{}", url);
                self.doc_ref_instance.add_docref(url).unwrap();
            }
            // Handle a list of strings in "doc_ref" section
            Value::Sequence(urls) => {
                for url_value in urls {
                    if let Value::String(url) = url_value {
                        self.doc_ref_instance.add_docref(url).unwrap();
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

    /// Prints the Ksy sections
    pub fn print_struct(&self) {
        println!("{:#?}", self.doc_instance);
        println!("{:#?}", self.doc_ref_instance);
    }
}
