/// Structure representing documentation for a Kaitai Struct.
#[derive(Debug)]
pub struct Doc {
    /// Description of the Kaitai struct.
    pub description: Option<String>,
}

impl Doc {
    /// Creates a new instance of `Doc` with no description.
    pub fn new() -> Self {
        Doc { description: None }
    }

    /// Sets the description for the Kaitai struct.
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }
}
