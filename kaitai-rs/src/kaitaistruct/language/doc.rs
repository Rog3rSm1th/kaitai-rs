use crate::kaitaistruct::language::kaitai_property::KaitaiProperty;

/// Doc property struct
#[derive(Debug)]
pub struct Doc {
    // Description of the Kaitai struct
    pub description: Option<String>,
}

impl Doc {
    pub fn new() -> Self {
        Doc { description: None }
    }

    // Sets the Doc description
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}