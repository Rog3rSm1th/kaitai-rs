use crate::config::config::Config;
use crate::errors::KaitaiError;
use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;
use regex::Regex;

/// DocRef property struct
#[derive(Debug)]
pub struct DocRef {
    // A doc ref element containing URL + arbitrary string
    pub content: Vec<DocRefElement>,
}

#[derive(Debug)]
pub struct DocRefElement {
    // DocRef URL
    pub url: Option<String>,
    // Arbitrary string
    pub arbitrary_string: Option<String>,
}

impl DocRef {
    pub fn new() -> Self {
        DocRef {
            content: Vec::new(),
        }
    }

    /// Adds a DocRefElement to the content vector with the specified URL.
    pub fn add_docref(&mut self, url: &str) -> Result<(), KaitaiError> {
        // Use DOCREF_PATTERN from Config to extract URL and arbitrary_string
        let docref_pattern = Regex::new(Config::DOCREF_PATTERN).unwrap();

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
            Err(KaitaiError::BadDocRef)
        }
    }
}
