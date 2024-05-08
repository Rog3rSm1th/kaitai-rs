use crate::ks_language::parser::parser::KSLanguageParser;

use serde_yaml;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

// Define a struct `FormatDescription` that wraps the `KSLanguageParser`
#[derive(Debug)]
pub struct FormatDescription {
    pub format: KSLanguageParser,
}

// Implement the `FormatDescription` struct with a `new` method and a `load_from_file` method
impl FormatDescription {
    /// Create a new instance of `FormatDescription`
    pub fn new() -> Self {
        FormatDescription {
            format: KSLanguageParser::new(),
        }
    }

    /// Load the format description from a YAML file and return a `FormatDescription` instance
    pub fn load_from_file<P: AsRef<Path>>(file_path: P) -> io::Result<FormatDescription> {
        // Open the file.
        let file = fs::File::open(file_path)?;

        // Create a buffered reader to efficiently read the file
        let mut reader = io::BufReader::new(file);

        // Read the file content into a String
        let mut content = String::new();
        reader.read_to_string(&mut content)?;

        // Parse the YAML content into serde_yaml::Value
        let yaml_value: serde_yaml::Value = serde_yaml::from_str(&content)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;

        // Create a new KSLanguageParser instance
        let mut parser = KSLanguageParser::new();

        // Parse the YAML content using the `KSLanguageParser`
        parser.parse_sections(&yaml_value)?;

        // Return a new FormatDescription instance with the parsed KSLanguageParser
        Ok(FormatDescription { format: parser })
    }
}
