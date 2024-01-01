use crate::config::Config;
use regex::Regex;
use std::io;

/// Struct representing a DocRef property
#[derive(Debug)]
pub struct DocRef {
    // A doc ref element containing URL + arbitrary string
    pub content: Vec<DocRefElement>,
}

/// Struct representing an element within a DocRef property
#[derive(Debug)]
pub struct DocRefElement {
    // DocRef URL
    pub url: Option<String>,
    // Arbitrary string
    pub arbitrary_string: Option<String>,
}

impl DocRef {
    /// Creates a new DocRef instance with an empty content vector.
    pub fn new() -> Self {
        DocRef {
            content: Vec::new(),
        }
    }

    /// Adds a DocRefElement to the content vector with the specified URL.
    pub fn add_docref(&mut self, url: &str) -> Result<(), io::Error> {
        // Use DOCREF_PATTERN from Config to extract URL and arbitrary_string
        let docref_pattern = Regex::new(Config::DOCREF_PATTERN).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidData, format!("Regex error: {}", e))
        })?;

        if let Some(captures) = docref_pattern.captures(url) {
            let url_part = captures.name("URL").map(|m| m.as_str().to_string());
            let arbitrary_part = captures
                .name("arbitrary_string")
                .map(|m| m.as_str().to_string());

            self.content.push(DocRefElement {
                url: url_part,
                arbitrary_string: arbitrary_part,
            });

            Ok(())
        } else {
            // Handle invalid doc-ref format
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Invalid doc-ref format",
            ))
        }
    }
}
